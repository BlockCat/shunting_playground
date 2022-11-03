mod evaluator;

use std::{hash::Hash, ops::Deref};

use mcts::{transposition_table::ApproxTable, tree_policy::UCTPolicy, GameState, MCTS};
use petgraph::visit::IntoNodeReferences;
use shunting_solution::Solution;

use self::evaluator::SolutionEvaluator;

#[derive(Debug, Clone, Default)]
pub struct SolutionWrapper(Solution);

impl Hash for SolutionWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (index, weight) in self.0.graph.node_references() {
            index.hash(state);
            weight.hash(state);
        }
    }
}

impl GameState for SolutionWrapper {
    type Move = ();
    type Player = ();
    type MoveList = Vec<Self::Move>;

    fn current_player(&self) -> Self::Player {
        
    }

    fn available_moves(&self) -> Self::MoveList {
        todo!()
    }

    fn make_move(&mut self, _mov: &Self::Move) -> Result<(), ()> {
        todo!()
    }

    fn get_winner(&self) -> Option<Self::Player> {
        None
    }
}

impl Deref for SolutionWrapper {
    type Target = Solution;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
struct ShuntingMCTS;

impl MCTS for ShuntingMCTS {
    type State = SolutionWrapper;

    type Eval = SolutionEvaluator;

    type TreePolicy = UCTPolicy<f64>;

    type NodeData = ();

    type TranspositionTable = ApproxTable<Self>;

    type ExtraThreadData = ();
}
