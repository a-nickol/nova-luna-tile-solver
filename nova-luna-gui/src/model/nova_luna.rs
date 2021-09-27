use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    game_board: HashMap<Position, Tile>,
}

impl State {
    pub fn new() -> State {
        Self {
            game_board: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize)]
pub struct Position(isize, isize);

#[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    color: Color,
    cost: usize,
    tasks: Vec<Task>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    colors: Vec<Color>,
    solved: bool,
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Blue,
    Yellow,
    Teal,
    Red,
}
