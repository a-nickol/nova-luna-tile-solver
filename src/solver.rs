use crate::nova_luna::{Move, State};
use mcts::transposition_table::ApproxTable;
use mcts::tree_policy::UCTPolicy;
use mcts::{CycleBehaviour, Evaluator, SearchHandle, MCTS};

pub struct StateEvaluator;

impl Evaluator<NovaLunaBoardGameMCTS> for StateEvaluator {
    type StateEvaluation = i64;

    fn evaluate_new_state(
        &self,
        state: &State,
        _moves: &Vec<Move>,
        _: Option<SearchHandle<NovaLunaBoardGameMCTS>>,
    ) -> (Vec<()>, i64) {
        (vec![()], state.count_solved_tasks() as i64)
    }

    fn interpret_evaluation_for_player(&self, evaln: &i64, _player: &()) -> i64 {
        *evaln
    }

    fn evaluate_existing_state(
        &self,
        _: &State,
        evaln: &i64,
        _: SearchHandle<NovaLunaBoardGameMCTS>,
    ) -> i64 {
        *evaln
    }
}

#[derive(Default)]
pub struct NovaLunaBoardGameMCTS;

impl MCTS for NovaLunaBoardGameMCTS {
    type State = State;
    type Eval = StateEvaluator;
    type NodeData = ();
    type ExtraThreadData = ();
    type TreePolicy = UCTPolicy;
    type TranspositionTable = ApproxTable<Self>;

    fn cycle_behaviour(&self) -> CycleBehaviour<Self> {
        CycleBehaviour::UseCurrentEvalWhenCycleDetected
    }
}
