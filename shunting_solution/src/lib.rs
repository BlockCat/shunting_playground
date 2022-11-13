#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use daggy::Dag;
use easy_error::Error;
use model::{PosAction, PosJson, PosTaskType};
use petgraph::{
    algo,
    graph::NodeReferences,
    stable_graph::{self, EdgeIndex},
    visit::IntoNodeReferences,
};
use shunting_location::{FacilityId, ShuntingYard, TrackPartYamlId, YardGraphIndex};
use std::io::Read;

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

    pub fn nodes(&self) -> NodeReferences<SolutionNode> {
        self.graph.node_references()
    }
    pub fn sorted_nodes(
        &self,
    ) -> Result<Vec<stable_graph::NodeIndex>, algo::Cycle<stable_graph::NodeIndex>> {
        algo::toposort(&self.graph, None)
    }

    pub fn validate(&self, yard: &ShuntingYard) -> Result<(), Vec<validate::SolutionConflict>> {
        validate_solution(self, yard)
    }
}

impl Solution {
    pub fn reverse_weak(mut self, edge: EdgeIndex) -> Result<Self, ()> {
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

pub type SolutionGraphIndex = u32;
// pub type SolutionGraph = DiGraph<SolutionNode, EdgeType, SolutionGraphIndex>;
pub type SolutionGraph = daggy::Dag<SolutionNode, EdgeType, SolutionGraphIndex>;

#[derive(Debug, Clone, Hash)]
pub struct SolutionNode {
    kind: NodeAction,
    train: Train,
}

#[derive(Debug, Clone, Hash)]
pub enum NodeAction {
    Arrival {
        arrival_time: ShuntingTime,
    },
    Departure {
        departure_time: ShuntingTime,
    },
    Movement {
        source: YardGraphIndex,
        destination: YardGraphIndex,
    },
    Task {
        kind: String,
        location: YardGraphIndex,
        facilities: Vec<FacilityId>,
        train_units: Vec<TrainUnit>,
        duration: ShuntingDuration,
    },
    Split {
        in_train: Train,
        out_trains: [Train; 2],
        duration: ShuntingDuration,
    },
    Combine {
        in_trains: [Train; 2],
        out_train: Train,
        duration: ShuntingDuration,
    },
    Turning {
        train: Train,
        duration: ShuntingDuration,
    },
}

/// Strong edges are edges between nodes that share trains and are directly scheduled after one another
/// Weak edges are edges between nodes that do not share trains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeType {
    Strong,
    Weak,
}

impl Solution {
    /// From json model to actually solution.
    pub fn from(model: PosJson, yard: &ShuntingYard) -> Self {
        let graph = model.json_graph();

        // for (index, node) in graph.node_references() {
        //     // let node = SolutionNode::from(node, yard);
        // }

        let toposorted = algo::toposort(&graph, None).expect("Cycle?");

        for index in toposorted {
            let weight = &graph[index];
            if let Some(task) = &weight.task {
                if task.kind == PosTaskType::Predefined(String::from("Arrive")) {
                    let [side_a, side_b] = yard.yaml_node(TrackPartYamlId(task.location));
                    
                }                
            }
        }

        unimplemented!()
    }
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
