use self::{
    facility::{Facility, FacilityId},
    rail::ShuntingRail,
    switch::ShuntingSwitch,
};
pub use crate::model::shunting_yard::TrackPartYamlId;
use crate::model::{
    read_yard,
    shunting_yard::{ShuntingYardYaml, TrackPart},
};
use petgraph::{
    stable_graph::{IndexType, NodeIndex},
    visit::IntoNodeReferences,
    Directed, Graph,
};
use std::{collections::HashMap, io::Read, ops::Index};
pub(crate) mod facility;
pub(crate) mod rail;
pub(crate) mod switch;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Clone, Copy)]
pub struct YardGraphIndex(pub u32);

unsafe impl IndexType for YardGraphIndex {
    fn new(x: usize) -> Self {
        Self(x as u32)
    }

    fn index(&self) -> usize {
        self.0 as usize
    }

    fn max() -> Self {
        Self(std::u32::MAX)
    }
}

pub type YardNodeIndex = NodeIndex<YardGraphIndex>;
pub type YardGraph = Graph<ShuntingRail, ShuntingSwitch, Directed, YardGraphIndex>;

type IntermediateGraph<'a> =
    Graph<(&'a TrackPart, RailDirection), ShuntingSwitch, Directed, YardGraphIndex>;
///
/// Shunting yard can be defined as a graph.
/// In this case, a rail is defined as two nodes in our network and a switch is an edge.
///
/// Rail:
/// - node for one way
/// - node for other way
///
pub struct ShuntingYard {
    pub graph: YardGraph,
    pub facilities: Vec<Facility>,

    // pub track_parts: Vec<TrackPart>,
    pub movement_constant: f32,
    pub movement_track_coefficient: f32,
    pub movement_switch_coefficient: f32,
}

impl ShuntingYard {
    pub fn read<R: Read>(reader: R) -> Self {
        let yaml = read_yard(reader).expect("Could not read yard");
        Self::from(yaml)
    }

    pub fn track_parts(&self) -> impl Iterator<Item = &TrackPart> {
        return self
            .graph
            .node_weights()
            .map(|x| -> &TrackPart { &x.track_part });
    }

    pub fn yaml_node(&self, find: TrackPartYamlId) -> [YardNodeIndex; 2] {
        let grr = self
            .graph
            .node_references()
            .filter(|(_, rail)| rail.track_part.id == find)
            .map(|x| x.0)
            .collect::<Vec<_>>();

        [grr[0], grr[1]]
    }

    pub fn yaml_facility(&self, find: usize) -> &Facility {
        self.facilities
            .iter()
            .find(|x| {
                dbg!(&x);
                dbg!(x.id.0) == dbg!(find as u8)
            })
            .unwrap()
    }
}

impl Index<FacilityId> for ShuntingYard {
    type Output = Facility;

    fn index(&self, index: FacilityId) -> &Self::Output {
        &self.facilities[index.0 as usize]
    }
}

impl From<ShuntingYardYaml> for ShuntingYard {
    fn from(yard: ShuntingYardYaml) -> Self {
        let facilities = load_facilities(&yard);
        let facility_map = facilities
            .iter()
            .map(|facility| (facility.name.clone(), facility.id))
            .collect::<HashMap<_, _>>();

        let (graph2, _) = build_intermediate_graph(&yard);

        let mut graph: YardGraph = graph2.map(
            |_, (tp, _)| ShuntingRail {
                id: tp.name.clone(),
                length: tp.length,
                saw_movement_allowed: tp.saw_movement_allowed,
                parking_allowed: tp.parking_allowed,
                facilities: connected_facilities(&facility_map, &yard, tp),
                track_part: (*tp).clone(),
            },
            |_, e| e.clone(),
        );

        graph.shrink_to_fit();
        Self {
            facilities,
            graph,
            movement_constant: yard.movement_constant,
            movement_switch_coefficient: yard.movement_switch_coefficient,
            movement_track_coefficient: yard.movement_track_coefficient,
        }
    }
}

fn load_facilities(yard: &ShuntingYardYaml) -> Vec<Facility> {
    yard.facilities
        .iter()
        .enumerate()
        .map(|(id, x)| {
            let id = FacilityId(id as u8);
            let name = x.id.clone();

            Facility { id, name }
        })
        .collect::<Vec<_>>()
}

fn connected_facilities(
    map: &HashMap<String, FacilityId>,
    yard: &ShuntingYardYaml,
    tp: &TrackPart,
) -> Vec<FacilityId> {
    yard.facilities
        .iter()
        .filter(|facility| facility.related_track_parts.contains(&tp.id))
        .map(|facility| map[&facility.id])
        .collect()
}

fn build_intermediate_graph(
    yard: &ShuntingYardYaml,
) -> (
    IntermediateGraph,
    HashMap<TrackPartYamlId, (YardNodeIndex, YardNodeIndex)>,
) {
    let trackpart_map = yard
        .track_parts
        .iter()
        .map(|x| (x.id, x))
        .collect::<HashMap<_, _>>();
    let mut graph: IntermediateGraph = IntermediateGraph::default();
    let nodes = yard
        .track_parts
        .iter()
        .map(|x| {
            let sidea = graph.add_node((x, RailDirection::SideA));
            let sideb = graph.add_node((x, RailDirection::SideB));

            (x.id, (sidea, sideb))
        })
        .collect::<HashMap<_, _>>();

    for (side_a, side_b) in nodes.values() {
        let part = graph[*side_a].0;
        if part.saw_movement_allowed {
            graph.add_edge(*side_a, *side_b, ShuntingSwitch::Rotation);
            graph.add_edge(*side_b, *side_a, ShuntingSwitch::Rotation);
        }

        for a_connection in &part.a_side {
            add_connection(
                &trackpart_map,
                a_connection,
                &nodes,
                part,
                &mut graph,
                side_a,
            );
        }

        for b_connection in &part.b_side {
            add_connection(
                &trackpart_map,
                b_connection,
                &nodes,
                part,
                &mut graph,
                side_b,
            );
        }
    }

    (graph, nodes)
}

fn add_connection(
    trackpart_map: &HashMap<TrackPartYamlId, &TrackPart>,
    other_id: &TrackPartYamlId,
    nodes: &HashMap<TrackPartYamlId, (YardNodeIndex, YardNodeIndex)>,
    part: &TrackPart,
    graph: &mut IntermediateGraph,
    node_index: &YardNodeIndex,
) {
    let other_part = trackpart_map[other_id];
    let other_nodes = nodes[other_id];
    if other_part.a_side.contains(&part.id) {
        graph.add_edge(*node_index, other_nodes.1, ShuntingSwitch::Switch);
    } else if other_part.b_side.contains(&part.id) {
        graph.add_edge(*node_index, other_nodes.0, ShuntingSwitch::Switch);
    } else {
        unreachable!()
    }
}

#[derive(Debug)]
enum RailDirection {
    SideA,
    SideB,
}
