use super::ShuntingMCTS;
use mcts::Evaluator;

pub struct SolutionEvaluator;

impl Evaluator<ShuntingMCTS> for SolutionEvaluator {
    type StateEvaluation = f64;

    fn evaluate_new_state(
        &self,
        _state: &<ShuntingMCTS as mcts::MCTS>::State,
        _moves: &mcts::MoveList<ShuntingMCTS>,
        _handle: Option<mcts::SearchHandle<ShuntingMCTS>>,
    ) -> (
        Vec<mcts::MoveEvaluation<ShuntingMCTS>>,
        Self::StateEvaluation,
    ) {
        todo!()
    }

    fn evaluate_existing_state(
        &self,
        _state: &<ShuntingMCTS as mcts::MCTS>::State,
        existing_evaln: &Self::StateEvaluation,
        _handle: mcts::SearchHandle<ShuntingMCTS>,
    ) -> Self::StateEvaluation {
        *existing_evaln
    }

    fn interpret_evaluation_for_player(
        &self,
        evaluation: &Self::StateEvaluation,
        _player: &mcts::Player<ShuntingMCTS>,
    ) -> f64 {
        *evaluation
    }
}
