#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use easy_error::Error;
use petgraph::{
    algo,
    graph::NodeReferences,
    stable_graph::{self, EdgeIndex},
    visit::IntoNodeReferences,
};
use shunting_location::ShuntingYard;
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
    let json = model::read_pos_json(reader)?;

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
        let json = model::read_pos_json(reader)?;

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
    pub fn update_times(mut self) -> Self {
        let toposort = algo::toposort(&self.graph, None).unwrap();

        for x in toposort {
            let node = &mut self.graph[x];
            match &mut node.kind {
                NodeAction::Arrival { arrival_time } => {}
                NodeAction::Departure { departure_time } => {}
                // NodeAction::Movement { start_time ,path, from_side, to_side, parking_side, order } => todo!(),
                // NodeAction::Task { start_time,kind, location, facilities, arrival_side, arrival_direction, departure_side, train_units } => todo!(),
                NodeAction::Split {
                    start_time,
                    in_train,
                    out_trains,
                } => todo!(),
                NodeAction::Combine {
                    start_time,
                    in_trains,
                    out_train,
                } => todo!(),
                NodeAction::Turning { start_time, train } => todo!(),
            }
        }
        unimplemented!("Should update times");

        self
    }
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

        Ok(self.update_times())
    }
}

pub type SolutionGraphIndex = u32;
// pub type SolutionGraph = DiGraph<SolutionNode, EdgeType, SolutionGraphIndex>;
pub type SolutionGraph = daggy::Dag<SolutionNode, EdgeType, SolutionGraphIndex>;

#[derive(Debug, Clone, Hash)]
pub struct SolutionNode {
    start_time: ShuntingTime,
    duration: ShuntingDuration,
    kind: NodeAction,
}

#[derive(Debug, Clone, Hash)]
pub enum NodeAction {
    Arrival {
        arrival_time: ShuntingTime,
    },
    Departure {
        departure_time: ShuntingTime,
    },
    // Movement {
    //     start_time: ShuntingTime,
    //     path: Vec<YardGraphIndex>,
    //     from_side: ?,
    //     to_side: ?,
    //     parking_side: ?
    //     order: usize,
    // },
    // Task {
    //     start_time: ShuntingTime,
    //     kind: TaskType,
    //     location: YardGraphIndex,
    //     facilities: Vec<FacilityId>,
    //     arrival_side: ?,
    //     arrival_direction: ?,
    //     departure_side: ?,
    //     train_units: Vec<TrainUnitId>
    // },
    Split {
        start_time: ShuntingTime,
        in_train: Train,
        out_trains: [Train; 2],
    },
    Combine {
        start_time: ShuntingTime,
        in_trains: [Train; 2],
        out_train: Train,
    },
    Turning {
        start_time: ShuntingTime,
        train: Train,
    },
}

/// Strong edges are edges between nodes that share trains and are directly scheduled after one another
/// Weak edges are edges between nodes that do not share trains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeType {
    Strong,
    Weak,
}
