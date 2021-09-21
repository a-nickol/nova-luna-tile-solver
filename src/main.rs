use mcts::transposition_table::*;
use mcts::tree_policy::*;
use mcts::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct Position(isize, isize);

#[derive(Copy, Clone, Hash, Debug, PartialEq)]
enum Color {
    Blue,
    Yellow,
    Teal,
    Red,
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Task {
    tiles: Vec<Color>,
    solved: bool,
}

impl Task {
    fn new(tiles: Vec<Color>) -> Task {
        Task {
            tiles,
            solved: false,
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq)]
struct Tile {
    color: Color,
    cost: usize,
    tasks: Vec<Task>,
}

impl Tile {
    fn new(cost: usize, color: Color, tasks: Vec<Task>) -> Tile {
        Tile { color, cost, tasks }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Move {
    tile: Tile,
    position: Position,
}

impl Move {
    #[cfg(test)]
    fn new(tile: Tile, position: Position) -> Move {
        Move { tile, position }
    }
}

#[derive(Clone, Debug)]
struct State {
    board: HashMap<Position, Tile>,
    tiles: Vec<Tile>,
}

impl State {
    #[cfg(test)]
    fn empty() -> State {
        State::with_tiles(vec![])
    }

    fn with_tiles(tiles: Vec<Tile>) -> State {
        State {
            board: HashMap::new(),
            tiles,
        }
    }
}

impl GameState for State {
    type Move = Move;
    type Player = ();
    type MoveList = Vec<Move>;

    fn current_player(&self) -> Self::Player {}

    fn available_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn make_move(&mut self, _mov: &Self::Move) {
        todo!()
    }
}

impl TranspositionHash for State {
    fn hash(&self) -> u64 {
        let mut h: u64 = 0;
        for elt in &self.board {
            let mut hasher = DefaultHasher::new();
            elt.hash(&mut hasher);
            h ^= hasher.finish();
        }
        let mut hasher = DefaultHasher::new();
        self.tiles.hash(&mut hasher);
        h ^= hasher.finish();
        h
    }
}

struct StateEvaluator;

impl Evaluator<NovaLunaBoardGameMCTS> for StateEvaluator {
    type StateEvaluation = i64;

    fn evaluate_new_state(
        &self,
        _state: &State,
        _moves: &Vec<Move>,
        _: Option<SearchHandle<NovaLunaBoardGameMCTS>>,
    ) -> (Vec<()>, i64) {
        todo!()
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
struct NovaLunaBoardGameMCTS;

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

fn main() {
    let unplaced_tiles = vec![
        Tile::new(
            6,
            Color::Yellow,
            vec![
                Task::new(vec![Color::Blue, Color::Blue, Color::Blue]),
                Task::new(vec![Color::Teal, Color::Teal]),
                Task::new(vec![Color::Teal, Color::Blue]),
            ],
        ),
        Tile::new(
            7,
            Color::Red,
            vec![
                Task::new(vec![Color::Teal, Color::Blue]),
                Task::new(vec![Color::Teal, Color::Yellow]),
                Task::new(vec![Color::Yellow, Color::Blue]),
            ],
        ),
        Tile::new(
            2,
            Color::Blue,
            vec![Task::new(vec![
                Color::Blue,
                Color::Blue,
                Color::Blue,
                Color::Blue,
            ])],
        ),
        Tile::new(
            2,
            Color::Yellow,
            vec![Task::new(vec![
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
            ])],
        ),
        Tile::new(
            4,
            Color::Teal,
            vec![
                Task::new(vec![Color::Blue, Color::Red]),
                Task::new(vec![Color::Teal, Color::Yellow]),
            ],
        ),
        Tile::new(
            5,
            Color::Teal,
            vec![
                Task::new(vec![Color::Red, Color::Red]),
                Task::new(vec![Color::Blue, Color::Blue]),
                Task::new(vec![Color::Teal, Color::Teal]),
            ],
        ),
        Tile::new(
            5,
            Color::Red,
            vec![
                Task::new(vec![Color::Yellow]),
                Task::new(vec![Color::Blue, Color::Blue]),
            ],
        ),
        Tile::new(
            5,
            Color::Yellow,
            vec![
                Task::new(vec![Color::Red]),
                Task::new(vec![Color::Teal, Color::Teal]),
            ],
        ),
        Tile::new(
            5,
            Color::Blue,
            vec![
                Task::new(vec![Color::Teal]),
                Task::new(vec![Color::Teal, Color::Teal, Color::Teal]),
            ],
        ),
        Tile::new(
            4,
            Color::Blue,
            vec![
                Task::new(vec![Color::Yellow, Color::Yellow]),
                Task::new(vec![Color::Teal, Color::Teal, Color::Teal]),
                Task::new(vec![Color::Red, Color::Red, Color::Red]),
            ],
        ),
        Tile::new(
            4,
            Color::Teal,
            vec![
                Task::new(vec![Color::Yellow, Color::Blue]),
                Task::new(vec![Color::Teal, Color::Teal, Color::Teal]),
            ],
        ),
    ];

    let game = State::with_tiles(unplaced_tiles);

    let mut mcts = MCTSManager::new(
        game,
        NovaLunaBoardGameMCTS,
        StateEvaluator,
        UCTPolicy::new(0.5),
        ApproxTable::new(1024),
    );

    mcts.playout_n_parallel(10000, 4);
    mcts.tree().debug_moves();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_state() {
        let state = State::empty();
        assert!(state.available_moves().is_empty());
    }

    #[test]
    fn single_tile() {
        let tile = Tile::new(1, Color::Yellow, vec![]);
        let state = State::with_tiles(vec![tile.clone()]);
        let moves = state.available_moves();
        assert_eq!(1, moves.len());
        assert_eq!(super::Move::new(tile, Position(0, 0)), moves[0]);
    }

    #[test]
    fn two_tiles() {
        let tile = Tile::new(1, Color::Yellow, vec![]);
        let mut state = State::with_tiles(vec![tile.clone(), tile.clone()]);

        state.make_move(&super::Move::new(tile.clone(), Position(0, 0)));

        let moves = state.available_moves();

        assert_eq!(4, moves.len());
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(1, 0))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(0, 1))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(-1, 0))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(0, -1))));
    }
}
