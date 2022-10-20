use std::time::SystemTime;

use petgraph::stable_graph::NodeIndex;

use crate::solution::{NodeType, SolutionRelation};

use super::Neighbourhood;

pub struct CombineMovementNeighbourhood;

impl Neighbourhood for CombineMovementNeighbourhood {
    type Context = ();
    type Action = (NodeIndex, NodeIndex);

    fn get_neighbourhood(
        solution: &crate::solution::Solution,
        after_time: SystemTime,
        _: &Self::Context,
    ) -> Vec<Self::Action> {
        let graph = &solution.graph;
        graph
            .raw_edges()
            .iter()
            .filter(|e| e.weight == SolutionRelation::TrainRelation)
            .filter(|e| match (&graph[e.source()], &graph[e.target()]) {
                (
                    NodeType::Movement {
                        route: _,
                        location: _,
                    },
                    NodeType::Movement {
                        route: _,
                        location: _,
                    },
                ) => true,
                _ => false,
            });
        todo!()
    }

    fn apply_action(
        solution: &crate::solution::Solution,
        action: Self::Action,
    ) -> Result<crate::solution::Solution, super::NeighbourhoodError> {
        todo!()
    }
}
