use mcts::transposition_table::TranspositionHash;
use mcts::GameState;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position(isize, isize);

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
pub enum Color {
    Blue,
    Yellow,
    Teal,
    Red,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Task {
    colors: Vec<Color>,
    solved: bool,
}

impl Task {
    pub fn new(colors: Vec<Color>) -> Task {
        Task {
            colors,
            solved: false,
        }
    }

    fn search_for_adjacent_tiles_matching_color(
        position: Position,
        color: Color,
        board: &HashMap<Position, Tile>,
    ) -> Vec<Position> {
        let mut visited_positions = HashSet::new();
        let mut unvisited_positions = vec![position];
        let mut adjacent_tiles = vec![];
        while let Some(pos) = unvisited_positions.pop() {
            if !visited_positions.contains(&pos) {
                visited_positions.insert(pos);
                if let Some(tile) = board.get(&pos) {
                    if tile.color == color {
                        adjacent_tiles.push(pos);
                        for p in pos.adjacent() {
                            unvisited_positions.push(p);
                        }
                    }
                }
            }
        }
        adjacent_tiles
    }

    fn is_solved(&self, pos: Position, state: &HashMap<Position, Tile>) -> bool {
        let mut map = HashMap::new();
        for p in pos.adjacent() {
            if let Some(tile) = state.get(&p) {
                let mut vec = Task::search_for_adjacent_tiles_matching_color(p, tile.color, state);
                let idx = vec.iter().position(|p| pos == *p);
                if let Some(idx) = idx {
                    vec.remove(idx);
                }
                let entry = map.entry(&tile.color).or_insert(0);
                *entry += vec.len();
            }
        }

        let mut found_colors = self.colors.clone();
        for (color, num) in map {
            for _ in 0..num {
                let found_color = found_colors.iter().position(|c| c == color);
                if let Some(idx) = found_color {
                    found_colors.remove(idx);
                }
            }
        }
        found_colors.is_empty()
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Tile {
    color: Color,
    cost: usize,
    tasks: Vec<Task>,
}

impl Tile {
    pub fn new(cost: usize, color: Color, tasks: Vec<Task>) -> Tile {
        Tile { color, cost, tasks }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    tile: Tile,
    position: Position,
}

impl Move {
    pub fn new(tile: Tile, position: Position) -> Move {
        Move { tile, position }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    board: HashMap<Position, Tile>,
    tiles: Vec<Tile>,
}

impl State {
    #[cfg(test)]
    fn empty() -> State {
        State::with_tiles(vec![])
    }

    pub fn with_tiles(tiles: Vec<Tile>) -> State {
        State {
            board: HashMap::new(),
            tiles,
        }
    }

    pub fn count_solved_tasks(&self) -> usize {
        self.board
            .iter()
            .flat_map(|(_pos, tile)| tile.tasks.iter())
            .filter(|tile| tile.solved)
            .count()
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

        for (pos, tile) in self.board.clone() {
            for (idx, task) in tile.tasks.iter().enumerate() {
                if !task.solved && task.is_solved(pos, &self.board) {
                    self.board.get_mut(&pos).unwrap().tasks[idx].solved = true;
                }
            }
        }
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

    #[test]
    fn solve_simple_task() {
        let tile1 = Tile::new(1, Color::Teal, vec![]);
        let tile2 = Tile::new(2, Color::Blue, vec![Task::new(vec![Color::Teal])]);

        let mut state = State::with_tiles(vec![tile1.clone(), tile2.clone()]);

        state.make_move(&super::Move::new(tile1.clone(), Position(0, 0)));
        state.make_move(&super::Move::new(tile2.clone(), Position(1, 0)));

        let option = state.board.get(&Position(1, 0));
        assert!(option.is_some());
        let tile = option.unwrap();

        let option = tile.tasks.get(0);
        assert!(option.is_some());
        let task = option.unwrap();

        assert!(task.solved);
    }

    #[test]
    fn solve_task_with_tile_group() {
        let tile1 = Tile::new(
            1,
            Color::Teal,
            vec![Task::new(vec![Color::Teal, Color::Teal])],
        );
        let tile2 = Tile::new(
            2,
            Color::Blue,
            vec![Task::new(vec![Color::Teal, Color::Teal])],
        );

        let mut state = State::with_tiles(vec![tile1.clone(), tile1.clone(), tile2.clone()]);

        state.make_move(&super::Move::new(tile1.clone(), Position(0, 0)));
        state.make_move(&super::Move::new(tile1.clone(), Position(1, 0)));
        state.make_move(&super::Move::new(tile2.clone(), Position(2, 0)));

        let tile = state.board.get(&Position(2, 0)).unwrap();
        let task = tile.tasks.get(0).unwrap();
        assert!(task.solved);

        let tile = state.board.get(&Position(0, 0)).unwrap();
        let task = tile.tasks.get(0).unwrap();
        assert!(!task.solved);

        assert_eq!(1, state.count_solved_tasks())
    }
}
