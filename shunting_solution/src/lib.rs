#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use easy_error::Error;
use petgraph::{
    algo,
    graph::NodeReferences,
    stable_graph::{self, EdgeIndex},
    visit::IntoNodeReferences,
};
use shunting_location::{FacilityId, ShuntingYard, YardGraphIndex};
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
                NodeAction::Movement {
                    destination,
                    source,
                } => todo!(),
                NodeAction::Task {
                    kind,
                    location,
                    facilities,
                    train_units,
                    duration,
                } => todo!(),
                NodeAction::Split {
                    in_train,
                    out_trains,
                    duration,
                } => todo!(),
                NodeAction::Combine {
                    in_trains,
                    out_train,
                    duration,
                } => todo!(),
                NodeAction::Turning { train, duration } => todo!(),
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
