use dipstick::InputScope;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use rayon::prelude::*;

use crate::{EdgeType, Solution, SHUNTING_SOLUTION};

pub trait Neighbourhood {
    fn neighbourhood(&self, solution: &Solution) -> Vec<Solution>;
}

pub struct ReverseOrderOfMovement;

impl Neighbourhood for ReverseOrderOfMovement {
    fn neighbourhood(&self, solution: &Solution) -> Vec<Solution> {
        SHUNTING_SOLUTION
            .marker("reverse_neighbourhood_search")
            .mark();
        SHUNTING_SOLUTION
            .timer("reverse_neighbourhood_calculation")
            .time(|| {
                // Get all weak edges
                let weak_edges = solution
                    .graph
                    .edge_references()
                    .filter(|x| x.weight() == &EdgeType::Weak)
                    .collect::<Vec<_>>();

                weak_edges
                    .into_par_iter()
                    .filter_map(|edge| solution.clone().reverse_weak(edge.id()).ok())
                    .collect()
            })
    }
}
