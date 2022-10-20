pub mod neighbourhood;

use crate::{train::Train, yard::layout::RailLocationIndex, ServiceType};
use petgraph::{stable_graph::DefaultIx, Graph};
use std::{
    error::Error,
    fmt::Display,
    time::{Duration, SystemTime},
};

pub const STANDARD_RAIL_TRAVERSE_DURATION: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
pub struct Solution {
    graph: Graph<SolutionNode, SolutionRelation>,
}

impl Solution {
    pub fn empty() -> Self {
        Self {
            graph: Default::default(),
        }
    }
    pub fn is_valid(&self) -> Result<(), Vec<SolutionError>> {
        if self.has_cycle() {
            return Err(vec![SolutionError::Cycle]);
        }

        // Test for shared rail
        // Test for overcapacitated services/rails
        // Test for inconsistencies (wrong locations, invalid service)
        // Test for

        Ok(())
    }

    fn has_cycle(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.graph)
    }
}

#[derive(Debug, Clone)]
pub struct SolutionNode {
    pub kind: NodeType,
    pub location: RailLocationIndex,
    pub start_time: SystemTime,
}
#[derive(Debug, Clone)]
pub enum NodeType {
    Movement { route: Vec<DefaultIx> },
    Service { service: ServiceType },
    Arrival { time: SystemTime },
    Departure { time: SystemTime },
    Turn,
    Split { result: [Train; 2] },
    Combine { result: Train },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutionRelation {
    TrainRelation,
    OtherRelation,
}

#[derive(Debug)]
pub enum SolutionError {
    Cycle,
    RailLocked(RailLocationIndex, Train, SolutionNode),
    LateDeparture(Train, SolutionNode),
    RailOvercapacitated(RailLocationIndex, Vec<Train>, SolutionNode),
    FacilityOvercapacitated(RailLocationIndex, Vec<Train>, SolutionNode),
    MissingService(Train, ServiceType),
    CombinationError(Train, Train, SolutionNode),
    TrainBlocksMovement(Train, Train, SolutionNode),
}

impl Display for SolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolutionError::Cycle => todo!(),
            SolutionError::RailLocked(_, _, _) => todo!(),
            SolutionError::LateDeparture(_, _) => todo!(),
            SolutionError::RailOvercapacitated(_, _, _) => todo!(),
            SolutionError::FacilityOvercapacitated(_, _, _) => todo!(),
            SolutionError::MissingService(_, _) => todo!(),
            SolutionError::CombinationError(_, _, _) => todo!(),
            SolutionError::TrainBlocksMovement(_, _, _) => todo!(),
        }
    }
}

impl Error for SolutionError {}
