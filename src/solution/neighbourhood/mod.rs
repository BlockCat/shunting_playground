use std::{error::Error, fmt::Display, time::SystemTime};

use super::Solution;

pub mod add_service;
pub mod remove_service;
pub mod reverse;
pub mod combine_movement;
pub mod combine_train;
pub mod split_train;


pub trait Neighbourhood {
    type Context;
    type Action;

    fn get_neighbourhood(solution: &Solution, after: SystemTime, context: &Self::Context) -> Vec<Self::Action>;
    fn apply_action(solution: &Solution, action: Self::Action) -> Result<Solution, NeighbourhoodError>;
}

#[derive(Debug)]
pub enum NeighbourhoodError {
    Cycle,
    Invalid,
}

impl Display for NeighbourhoodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Neighbourhood error: ")?;
        match self {
            NeighbourhoodError::Cycle => f.write_str("Cycle\n"),
            NeighbourhoodError::Invalid => f.write_str("Invalid\n"),
        }
    }
}

impl Error for NeighbourhoodError {}
