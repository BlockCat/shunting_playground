#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use std::{io::Read, time::{Duration, SystemTime}};
use easy_error::Error;
use petgraph::{prelude::DiGraph, stable_graph, algo, visit::IntoNodeReferences, graph::NodeReferences};
use shunting_location::{FacilityId, YardGraphIndex, ShuntingYard};

pub mod model;

mod validate;

use validate::validate_solution;

pub fn read_solution<R: Read>(reader: R) -> Result<Solution, Error> {
    let json = model::read_pos_json(reader)?;

    unimplemented!()
}

/// A Solution is basically a graph, 
/// We want to verify a solution. Score a solution
/// Nodes are stored in the smallest units. A movement with a rotation is a: (movement, rotation, movement)
#[derive(Debug, Clone)]
pub struct Solution {
    pub graph: SolutionGraph
}

impl Solution {    
    pub fn read<R: Read>(reader: R) -> Result<Self, Error> {
        let json = model::read_pos_json(reader)?;

        unimplemented!()
    }

    pub fn nodes(&self) -> NodeReferences<SolutionNode>  {
        self.graph.node_references()
    }
    pub fn sorted_nodes(&self) -> Result<Vec<stable_graph::NodeIndex>, algo::Cycle<stable_graph::NodeIndex>> {
        algo::toposort(&self.graph, None)
    }

    pub fn validate(&self, yard: &ShuntingYard) -> Result<(), Vec<validate::SolutionConflict>> {
        validate_solution(self, yard)
    }
}

pub type SolutionGraphIndex = u32;
pub type SolutionGraph = DiGraph<SolutionNode, EdgeType, SolutionGraphIndex>;

#[derive(Debug, Clone)]
pub struct SolutionNode {
    start_time: usize,
    duration: usize,
    kind: NodeAction,
}

#[derive(Debug, Clone)]
pub enum NodeAction {
    Arrival {

    },
    Departure {

    },
    Movement {
        path: Vec<YardGraphIndex>,
        from_side: ?,
        to_side: ?,
        parking_side: ?
        order: usize,
    },    
    Task {
        kind: TaskType,
        location: YardGraphIndex,
        facilities: Vec<FacilityId>,
        arrival_side: ?,
        arrival_direction: ?,
        departure_side: ?,
        train_units: Vec<TrainUnitId>
    }, 
    Split {

    },
    Combine {

    },
    Turning {

    }
}

/// Strong edges are edges between nodes that share trains and are directly scheduled after one another
/// Weak edges are edges between nodes that do not share trains.
#[derive(Debug, Clone)]
pub enum EdgeType {
    Strong,
    Weak,
}