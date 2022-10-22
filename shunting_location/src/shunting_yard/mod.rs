pub(crate) mod facility;
pub(crate) mod rail;
pub(crate) mod switch;

use self::{
    facility::{Facility, FacilityId},
    rail::ShuntingRail,
    switch::ShuntingSwitch,
};
use crate::model::{
    read_yard,
    shunting_yard::{RailType, ShuntingYardYaml, TrackPartYamlId},
};
use petgraph::{stable_graph::NodeIndex, Graph};
use std::{collections::HashMap, io::Read, ops::Index};

///
/// Shunting yard can be defined as a graph.
/// In this case, a rail is defined as two nodes in our network and a switch is an edge.
///
/// Rail:
/// - node for one way
/// - node for other way
///
pub struct ShuntingYard {
    pub graph: Graph<ShuntingRail, ShuntingSwitch>,
    pub facilities: Vec<Facility>,
}

impl ShuntingYard {
    pub fn read<R: Read>(reader: R) -> Self {
        let yaml = read_yard(reader).expect("Could not read yard");
        Self::from(yaml)
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

        let mut graph = petgraph::Graph::new();

        let node_map = create_rail_nodes(&yard, &mut graph)
            .into_iter()
            .map(|x| (x.yaml_id, x))
            .collect::<HashMap<_, _>>();

        // connect all sideA to sideB
        connect_switches(&yard, &mut graph, &node_map);
        // connect_bumpers(&yard, &mut graph, &node_map);
        connect_intersections(&yard, &mut graph);

        Self { facilities, graph }
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

fn create_rail_nodes(
    yard: &ShuntingYardYaml,
    graph: &mut Graph<ShuntingRail, ShuntingSwitch>,
) -> Vec<RailNodeEntry> {
    yard.track_parts
        .iter()
        .filter(|x| x.kind == RailType::RailRoad)
        .map(|part| {
            let track = ShuntingRail::Rail {
                id: part.name.clone(),
                facilities: Default::default(),
                length: part.length,
                parking_allowed: part.parking_allowed,
                saw_movement_allowed: part.saw_movement_allowed,
            };

            let switch = ShuntingSwitch::Rotation;

            let side_a = graph.add_node(track.clone());
            let side_b = graph.add_node(track);

            if part.saw_movement_allowed {
                graph.add_edge(side_a, side_b, switch.clone());
                graph.add_edge(side_b, side_a, switch);
            }

            RailNodeEntry {
                yaml_id: part.id,
                side_a,
                side_b,
                side_a_connections: part.a_side.clone(),
                side_b_connections: part.b_side.clone(),
                bumper: false,
            }
        })
        .collect()
}

fn connect_switches(
    yard: &ShuntingYardYaml,
    graph: &mut Graph<ShuntingRail, ShuntingSwitch>,
    map: &HashMap<TrackPartYamlId, RailNodeEntry>,
) {
    let switches = yard
        .track_parts
        .iter()
        .filter(|x| x.kind == RailType::Switch || x.kind == RailType::EnglishSwitch);

    for switch in switches {
        for a in &switch.a_side {
            for b in &switch.b_side {
                // connect a[node][forward] -> b[node][forward]
                // connect b[node][backward] -> a[node][backward]
                let node_a = map.get(a);
                let node_b = map.get(b);

                unimplemented!("We are going to be needing some handshake implementation: If you have me, and i have you.")

                match (node_a, node_b) {
                    (Some(node_a), Some(node_b)) => {
                        if node_a.bumper || node_b.bumper {
                            graph.add_edge(node_b.side_b, node_a.side_a, ShuntingSwitch::Bumper);
                            graph.add_edge(node_a.side_b, node_b.side_a, ShuntingSwitch::Bumper);
                        } else {
                            graph.add_edge(node_b.side_b, node_a.side_b, ShuntingSwitch::Switch);
                            graph.add_edge(node_a.side_a, node_b.side_a, ShuntingSwitch::Switch);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn connect_intersections(yard: &ShuntingYardYaml, graph: &mut Graph<ShuntingRail, ShuntingSwitch>) {
    // todo!();
}

#[derive(Debug)]
struct RailNodeEntry {
    yaml_id: TrackPartYamlId,
    side_a: NodeIndex,
    side_a_connections: Vec<TrackPartYamlId>,
    side_b: NodeIndex,
    side_b_connections: Vec<TrackPartYamlId>,
    bumper: bool,
}
