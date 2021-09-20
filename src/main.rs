use mcts::transposition_table::*;
use mcts::tree_policy::*;
use mcts::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hasher, Hash};

#[derive(Clone, Copy, Debug, Hash)]
struct Position(usize, usize);

#[derive(Copy, Clone, Hash, Debug)]
enum Color {
    Blue,
    Yellow,
    Teal,
    Red,
}

#[derive(Clone, Debug, Hash)]
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

#[derive(Clone, Hash, Debug)]
struct Tile {
    color: Color,
    cost: usize,
    tasks: Vec<Task>,
}

impl Tile {
    fn new(color: Color, cost: usize, tasks: Vec<Task>) -> Tile {
        Tile { color, cost, tasks }
    }
}

#[derive(Clone, Debug)]
struct Move {
    tile: Tile,
    position: Position,
}

#[derive(Clone, Debug)]
struct State {
    placed_tiles: HashMap<Position, Tile>,
    unplaced_tiles: Vec<Tile>,
}

impl GameState for State {
    type Move = Move;
    type Player = ();
    type MoveList = Vec<Move>;

    fn current_player(&self) -> Self::Player {
        ()
    }

    fn available_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn make_move(&mut self, _mov: &Self::Move) {
        todo!()
    }
}

impl TranspositionHash for State {
    fn hash(&self) -> u64 {
        let mut h :u64 = 0;
        for elt in &self.placed_tiles {
            let mut hasher = DefaultHasher::new();
            elt.hash(&mut hasher);
            h ^= hasher.finish();
        }
        let mut hasher = DefaultHasher::new();
        self.unplaced_tiles.hash(&mut hasher);
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
    let mut unplaced_tiles = vec![];

    unplaced_tiles.push(Tile::new(
        Color::Yellow,
        6,
        vec![
            Task::new(vec![Color::Blue, Color::Blue, Color::Blue]),
            Task::new(vec![Color::Teal, Color::Teal]),
            Task::new(vec![Color::Teal, Color::Blue]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Red,
        7,
        vec![
            Task::new(vec![Color::Teal, Color::Blue]),
            Task::new(vec![Color::Teal, Color::Yellow]),
            Task::new(vec![Color::Yellow, Color::Blue]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Blue,
        2,
        vec![Task::new(vec![
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
        ])],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Yellow,
        2,
        vec![Task::new(vec![
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
            Color::Yellow,
        ])],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Teal,
        4,
        vec![
            Task::new(vec![Color::Blue, Color::Red]),
            Task::new(vec![Color::Teal, Color::Yellow]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Teal,
        5,
        vec![
            Task::new(vec![Color::Red, Color::Red]),
            Task::new(vec![Color::Blue, Color::Blue]),
            Task::new(vec![Color::Teal, Color::Teal]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Red,
        5,
        vec![
            Task::new(vec![Color::Yellow]),
            Task::new(vec![Color::Blue, Color::Blue]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Yellow,
        5,
        vec![
            Task::new(vec![Color::Red]),
            Task::new(vec![Color::Teal, Color::Teal]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Blue,
        5,
        vec![
            Task::new(vec![Color::Teal]),
            Task::new(vec![Color::Teal, Color::Teal, Color::Teal]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Blue,
        4,
        vec![
            Task::new(vec![Color::Yellow, Color::Yellow]),
            Task::new(vec![Color::Teal, Color::Teal, Color::Teal]),
            Task::new(vec![Color::Red, Color::Red, Color::Red]),
        ],
    ));

    unplaced_tiles.push(Tile::new(
        Color::Teal,
        4,
        vec![
            Task::new(vec![Color::Yellow, Color::Blue]),
            Task::new(vec![Color::Teal, Color::Teal, Color::Teal]),
        ],
    ));

    let game = State {
        placed_tiles: HashMap::new(),
        unplaced_tiles,
    };

    let mut mcts = MCTSManager::new(
        game,
        NovaLunaBoardGameMCTS,
        StateEvaluator,
        UCTPolicy::new(0.5),
        ApproxTable::new(1024),
    );

    mcts.playout_n_parallel(10000, 4); // 10000 playouts, 4 search threads
    mcts.tree().debug_moves();
}
