mod nova_luna;
mod solver;

use crate::nova_luna::{Color, State, Task, Tile};
use crate::solver::NovaLunaBoardGameMCTS;
use crate::solver::StateEvaluator;
use mcts::transposition_table::*;
use mcts::tree_policy::*;
use mcts::*;
use std::time::Instant;

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

    let start = Instant::now();

    let num_tiles = unplaced_tiles.len();
    let game = State::with_tiles(unplaced_tiles.clone());

    let mut mcts = MCTSManager::new(
        game,
        NovaLunaBoardGameMCTS,
        StateEvaluator,
        UCTPolicy::new(2.0),
        ApproxTable::new(1024),
    );

    mcts.playout_n_parallel(100000, 4);

    let mut game = State::with_tiles(unplaced_tiles);
    for m in mcts.principal_variation(num_tiles) {
        game.make_move(&m);
        eprintln!("{:?}", m);
    }
    eprintln!("{}", game.count_solved_tasks());

    eprintln!("elapsed: {:?}", start.elapsed());
}
