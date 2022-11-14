#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use daggy::{Dag, NodeIndex};
use easy_error::Error;
use model::{PosAction, PosJson, PosTaskType};
use petgraph::{
    algo,
    graph::NodeReferences,
    stable_graph::{self, EdgeIndex, IndexType},
    visit::{IntoNeighborsDirected, IntoNodeReferences},
};
use shunting_location::{FacilityId, ShuntingYard, TrackPartYamlId, YardGraphIndex};
use std::{collections::VecDeque, io::Read};

pub use train::*;

pub mod model;
mod neighbourhood;
pub mod time;
mod train;
mod validate;

use time::{ShuntingDuration, ShuntingTime};
use validate::validate_solution;

pub fn read_solution<R: Read>(reader: R) -> Result<Solution, Error> {
    let _json = model::read_pos_json(reader)?;

    unimplemented!()
}

/// A Solution is basically a graph,
/// Nodes are stored in the smallest units. A movement with a rotation is a: (movement, rotation, movement)
/// The solution should be able to check if it is valid.
#[derive(Debug, Clone, Default)]
pub struct Solution {
    pub graph: SolutionGraph,
}

impl Solution {
    pub fn read<R: Read>(reader: R) -> Result<Self, Error> {
        let _json = model::read_pos_json(reader)?;

        unimplemented!()
    }

    pub fn nodes(&self) -> NodeReferences<SolutionNode, SolutionGraphIndex> {
        self.graph.node_references()
    }
    pub fn sorted_nodes(&self) -> Result<Vec<SolutionNodeIndex>, algo::Cycle<SolutionNodeIndex>> {
        algo::toposort(&self.graph, None)
    }

    pub fn validate(&self, yard: &ShuntingYard) -> Result<(), Vec<validate::SolutionConflict>> {
        validate_solution(self, yard)
    }
}

impl Solution {
    pub fn reverse_weak(mut self, edge: SolutionEdgeIndex) -> Result<Self, ()> {
        debug_assert!(
            self.graph[edge] == EdgeType::Weak,
            "Can only reverse weak edges"
        );
        debug_assert!(
            !algo::is_cyclic_directed(&self.graph),
            "uh oh, cycle detected"
        );

        let (source, target) = self.graph.edge_endpoints(edge).unwrap();

        self.graph.remove_edge(edge);
        self.graph
            .add_edge(target, source, EdgeType::Weak)
            .expect("Could not add edge (cycle)");

        Ok(self)
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SolutionGraphIndex(u32);

pub type SolutionNodeIndex = NodeIndex<SolutionGraphIndex>;
pub type SolutionEdgeIndex = EdgeIndex<SolutionGraphIndex>;

unsafe impl IndexType for SolutionGraphIndex {
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

pub type SolutionGraph = daggy::Dag<SolutionNode, EdgeType, SolutionGraphIndex>;

#[derive(Debug, Clone, Hash)]
pub struct SolutionNode {
    kind: NodeAction,
    train: Train,
}

impl SolutionNode {
    pub fn edge_relation(&self, other: &Self) -> EdgeType {
        if self.train.overlaps(&other.train) {
            EdgeType::Strong
        } else {
            EdgeType::Weak
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum NodeAction {
    Arrival {
        arrival_time: ShuntingTime,
        location: NodeIndex<YardGraphIndex>,
    },
    Departure {
        departure_time: ShuntingTime,
    },
    Movement {
        source: NodeIndex<YardGraphIndex>,
        destination: NodeIndex<YardGraphIndex>,
    },
    Task {
        kind: String,
        location: NodeIndex<YardGraphIndex>,
        facilities: Vec<FacilityId>,
        train_units: Vec<TrainUnit>,
        duration: ShuntingDuration,
    },
    Split {
        in_train: Train,
        out_trains: [Train; 2],
        duration: ShuntingDuration,
        location: NodeIndex<YardGraphIndex>,
    },
    Combine {
        in_trains: [Train; 2],
        out_train: Train,
        duration: ShuntingDuration,
        location: NodeIndex<YardGraphIndex>,
    },
    Turning {
        train: Train,
        duration: ShuntingDuration,
        location: NodeIndex<YardGraphIndex>,
    },
}

/// Strong edges are edges between nodes that share trains and are directly scheduled after one another
/// Weak edges are edges between nodes that do not share trains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeType {
    Strong,
    Weak,
}

struct ParentEntry {
    json_node: NodeIndex,
    solution_node: SolutionNodeIndex,
}
impl Solution {
    /// From json model to actually solution.
    pub fn from(model: PosJson, yard: &ShuntingYard) -> Self {
        // for (index, node) in graph.node_references() {
        //     // let node = SolutionNode::from(node, yard);
        // }

        let json_graph = model.json_graph();

        let mut solution_graph = SolutionGraph::new();

        let mut stack = json_graph
            .node_references()
            .filter(|(_, pos)| {
                if let Some(task) = &pos.task {
                    task.kind == PosTaskType::Predefined(String::from("Arrive"))
                } else {
                    false
                }
            })
            .map(|x| (None, x.0))
            .collect::<VecDeque<_>>();

        while let Some((parent, current_json_node)) = stack.pop_back() {
            let weight = &json_graph[current_json_node];
            let train = Train::new(&weight.train_unit_ids);
            let node = if let Some(task) = &weight.task {
                if task.kind == PosTaskType::Predefined(String::from("Arrive")) {
                    continue_arrive_node(
                        yard,
                        task,
                        weight,
                        train,
                        &mut solution_graph,
                        &json_graph,
                        current_json_node,
                        &mut stack,
                    )
                } else if let PosTaskType::Other(other_task) = &task.kind {
                    let node = SolutionNode {
                        train,
                        kind: NodeAction::Task {
                            kind: other_task.clone(),
                            location: yard.yaml_node(TrackPartYamlId(task.location))[0],
                            facilities: task
                                .facilities
                                .iter()
                                .map(|x| yard.yaml_facility(x.id).id)
                                .collect(),
                            train_units: task
                                .train_unit_ids
                                .iter()
                                .map(|x| TrainUnit(*x))
                                .collect(),
                            duration: ShuntingDuration::new(weight.minimum_duration as usize),
                        },
                    };

                    let node_index = solution_graph.add_node(node.clone());

                    add_children(&json_graph, current_json_node, &mut stack, node_index);
                    (node, node_index)
                } else {
                    unimplemented!("Unimplemented predefined task: {:?}", task.kind)
                }
            } else if let Some(movement) = &weight.movement {
                // let parent = parent.expect("Should have a parent node");
                // let parent_weight = &solution_graph[parent.solution_node];

                let path = movement
                    .path
                    .iter()
                    .map(|node| yard.yaml_node(TrackPartYamlId(*node)))
                    .collect::<Vec<_>>();

                assert!(path.len() >= 2);

                let source = {
                    let first = path[0];
                    let first_after = path[1];

                    first
                        .into_iter()
                        .filter(|x| {
                            yard.graph
                                .neighbors_directed(*x, petgraph::Direction::Outgoing)
                                .find(|x| x == &first_after[0] || x == &first_after[1])
                                .is_some()
                        })
                        .next()
                        .unwrap()
                };

                let destination = {
                    let last = path[path.len() - 1];
                    let last_before = path[path.len() - 2];

                    last.into_iter()
                        .filter(|x| {
                            yard.graph
                                .neighbors_directed(*x, petgraph::Direction::Outgoing)
                                .find(|x| x == &last_before[0] || x == &last_before[1])
                                .is_some()
                        })
                        .next()
                        .unwrap()
                };

                let node = SolutionNode {
                    train,
                    kind: NodeAction::Movement {
                        source,
                        destination,
                    },
                };

                let node_index = solution_graph.add_node(node.clone());

                add_children(&json_graph, current_json_node, &mut stack, node_index);

                (node, node_index)
            } else {
                unimplemented!("Unimplemented anything: {:?}", weight)
            };

            if let Some(parent) = parent {
                let parent_weight = &solution_graph[parent.solution_node];
                let child_weight = &node.0;
                solution_graph
                    .add_edge(
                        parent.solution_node,
                        node.1,
                        parent_weight.edge_relation(child_weight),
                    )
                    .unwrap();
            }
        }

        unimplemented!("Queue empty")
    }
}

fn add_children(
    json_graph: &Dag<PosAction, f32>,
    current_json_node: NodeIndex,
    stack: &mut VecDeque<(Option<ParentEntry>, NodeIndex)>,
    node_index: SolutionNodeIndex,
) {
    for i in json_graph.neighbors_directed(current_json_node, petgraph::Direction::Outgoing) {
        stack.push_front((
            Some(ParentEntry {
                json_node: current_json_node,
                solution_node: node_index,
            }),
            i,
        ));
    }
}

fn continue_arrive_node(
    yard: &ShuntingYard,
    task: &model::PosTask,
    weight: &PosAction,
    train: Train,
    solution_graph: &mut SolutionGraph,
    json_graph: &Dag<PosAction, f32>,
    current_json_node: NodeIndex,
    stack: &mut VecDeque<(Option<ParentEntry>, NodeIndex)>,
) -> (SolutionNode, SolutionNodeIndex) {
    let yard_side_a = yard.yaml_node(TrackPartYamlId(task.location))[0];
    let node = SolutionNode {
        kind: NodeAction::Arrival {
            arrival_time: ShuntingTime::new(weight.suggested_starting_time),
            location: yard_side_a,
        },
        train,
    };
    let node_index = solution_graph.add_node(node.clone());
    add_children(json_graph, current_json_node, stack, node_index);
    (node, node_index)
}

// impl SolutionNode {
//     fn from(action: &PosAction, yard: &ShuntingYard) -> Self {
//         let train = Train {
//             units: action
//                 .train_unit_ids
//                 .iter()
//                 .map(|&id| TrainUnit(id))
//                 .collect(),
//         };
//         if let Some(movement) = action.movement {
//             let source = *movement.path.first().unwrap();
//             let target = *movement.path.last().unwrap();
//             Self {
//                 train,
//                 kind: NodeAction::Movement {
//                     source: todo!("Get source from yard location. (direction)"),
//                     destination: todo!(),
//                 },
//             }
//         }
//     }
// }
