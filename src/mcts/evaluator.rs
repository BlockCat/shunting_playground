use super::ShuntingMCTS;
use mcts::Evaluator;

pub struct SolutionEvaluator;

impl Evaluator<ShuntingMCTS> for SolutionEvaluator {
    type StateEvaluation = f64;

    fn evaluate_new_state(
        &self,
        state: &<ShuntingMCTS as mcts::MCTS>::State,
        moves: &mcts::MoveList<ShuntingMCTS>,
        handle: Option<mcts::SearchHandle<ShuntingMCTS>>,
    ) -> (
        Vec<mcts::MoveEvaluation<ShuntingMCTS>>,
        Self::StateEvaluation,
    ) {
        todo!()
    }

    fn evaluate_existing_state(
        &self,
        state: &<ShuntingMCTS as mcts::MCTS>::State,
        existing_evaln: &Self::StateEvaluation,
        handle: mcts::SearchHandle<ShuntingMCTS>,
    ) -> Self::StateEvaluation {
        *existing_evaln
    }

    fn interpret_evaluation_for_player(
        &self,
        evaluation: &Self::StateEvaluation,
        player: &mcts::Player<ShuntingMCTS>,
    ) -> f64 {
        *evaluation
    }
}
