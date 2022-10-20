use std::time::SystemTime;

use super::{Neighbourhood, NeighbourhoodError};
use crate::solution::SolutionRelation;
use petgraph::stable_graph::NodeIndex;

pub struct ReverseNeighbourhood;

impl Neighbourhood for ReverseNeighbourhood {
    type Context = ();

    type Action = (NodeIndex, NodeIndex);

    fn get_neighbourhood(
        solution: &crate::solution::Solution,
        after_time: SystemTime,
        _: &Self::Context,
    ) -> Vec<Self::Action> {
        solution
            .graph
            .raw_edges()
            .iter()
            .filter(|x| x.weight == SolutionRelation::OtherRelation)
            .map(|relation| (relation.source(), relation.target()))
            .collect()
    }

    fn apply_action(
        solution: &crate::solution::Solution,
        action: Self::Action,
    ) -> Result<crate::solution::Solution, NeighbourhoodError> {
        let mut solution = solution.clone();

        match solution.graph.find_edge(action.0, action.1) {
            Some(edge) => {
                solution.graph.remove_edge(edge);
                solution
                    .graph
                    .add_edge(action.1, action.0, SolutionRelation::OtherRelation);
            }
            _ => return Err(NeighbourhoodError::Invalid),
        }

        let valid = solution.is_valid();
        match valid {
            Ok(_) => Ok(solution),
            Err(err) => Err(NeighbourhoodError::Invalid(err)),
        }
    }
}
