use mcts::transposition_table::*;
use mcts::tree_policy::*;
use mcts::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position(isize, isize);

impl Position {
    fn adjacent(&self) -> [Position; 4] {
        let mut result = [Position(0, 0); 4];
        result[0] = self.offset(1, 0);
        result[1] = self.offset(0, 1);
        result[2] = self.offset(-1, 0);
        result[3] = self.offset(0, -1);
        result
    }

    fn offset(&self, x: isize, y: isize) -> Position {
        Position(self.0 + x, self.1 + y)
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
enum Color {
    Blue,
    Yellow,
    Teal,
    Red,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Move {
    tile: Tile,
    position: Position,
}

impl Move {
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
        let empty_positions = if self.board.is_empty() {
            vec![Position(0, 0)]
        } else {
            self.board
                .iter()
                .flat_map(|(pos, _)| (*pos).adjacent())
                .filter(|pos| !self.board.contains_key(pos))
                .collect()
        };

        let set: HashSet<Move> = self
            .tiles
            .iter()
            .flat_map(|t| empty_positions.iter().copied().zip(iter::repeat(t)))
            .map(|(pos, tile): (Position, &Tile)| Move::new(tile.clone(), pos))
            .collect();

        set.into_iter().collect()
    }

    fn make_move(&mut self, mov: &Self::Move) {
        let idx = self
            .tiles
            .iter()
            .position(|t| *t == mov.tile)
            .expect("cannot find played tile");
        self.tiles.remove(idx);
        self.board.insert(mov.position, mov.tile.clone());
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

    #[test]
    fn three_tiles() {
        let tile = Tile::new(1, Color::Yellow, vec![]);
        let mut state = State::with_tiles(vec![tile.clone(), tile.clone(), tile.clone()]);

        state.make_move(&super::Move::new(tile.clone(), Position(0, 0)));
        state.make_move(&super::Move::new(tile.clone(), Position(1, 0)));

        let moves = state.available_moves();

        assert_eq!(6, moves.len());

        assert!(moves.contains(&super::Move::new(tile.clone(), Position(2, 0))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(1, 1))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(1, -1))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(0, 1))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(-1, 0))));
        assert!(moves.contains(&super::Move::new(tile.clone(), Position(0, -1))));
    }

    #[test]
    fn two_available_tiles() {
        let tile1 = Tile::new(1, Color::Yellow, vec![]);
        let tile2 = Tile::new(2, Color::Blue, vec![Task::new(vec![Color::Teal])]);

        let mut state = State::with_tiles(vec![
            tile1.clone(),
            tile1.clone(),
            tile1.clone(),
            tile2.clone(),
        ]);

        state.make_move(&super::Move::new(tile1.clone(), Position(0, 0)));
        state.make_move(&super::Move::new(tile1.clone(), Position(1, 0)));

        let moves = state.available_moves();

        assert_eq!(12, moves.len());

        assert!(moves.contains(&super::Move::new(tile1.clone(), Position(2, 0))));
        assert!(moves.contains(&super::Move::new(tile1.clone(), Position(1, 1))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(2, 0))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(1, 1))));
    }

    #[test]
    fn four_tiles() {
        let tile1 = Tile::new(1, Color::Yellow, vec![]);
        let tile2 = Tile::new(2, Color::Blue, vec![Task::new(vec![Color::Teal])]);

        let mut state = State::with_tiles(vec![
            tile1.clone(),
            tile1.clone(),
            tile1.clone(),
            tile2.clone(),
        ]);

        state.make_move(&super::Move::new(tile1.clone(), Position(0, 0)));
        state.make_move(&super::Move::new(tile1.clone(), Position(1, 0)));
        state.make_move(&super::Move::new(tile1.clone(), Position(0, -1)));

        let moves = state.available_moves();

        assert_eq!(7, moves.len());

        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(2, 0))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(1, 1))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(1, 1))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(1, -1))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(0, 1))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(-1, -1))));
        assert!(moves.contains(&super::Move::new(tile2.clone(), Position(0, -2))));
    }
}
