mod nova_luna;
mod solver;

use crate::nova_luna::{State, Tile};
use crate::solver::NovaLunaBoardGameMCTS;
use crate::solver::StateEvaluator;
use chrono::Utc;
use mcts::transposition_table::*;
use mcts::tree_policy::*;
use mcts::*;
use std::io::Write;
use std::time::Instant;

fn main() {
    let unplaced_tiles = std::fs::read_to_string("resources/tiles.json").expect("cannot read file");
    let unplaced_tiles: Vec<Tile> =
        serde_json::from_str(unplaced_tiles.as_str()).expect("cannot parse tiles");

    let now = Instant::now();
    let num_tiles = unplaced_tiles.len();

    let state = State::with_tiles(unplaced_tiles);
    let mut mcts = MCTSManager::new(
        state.clone(),
        NovaLunaBoardGameMCTS,
        StateEvaluator,
        UCTPolicy::new(2.0),
        ApproxTable::new(1024),
    );

    let num_playouts = 100;
    eprintln!("Doing {} playouts.", num_playouts);
    eprintln!();

    mcts.playout_n_parallel(num_playouts, 4);

    eprintln!("Best Moves:");
    eprintln!("-----------");
    let mut game = state;
    for m in mcts.principal_variation(num_tiles) {
        game.make_move(&m);
        eprintln!("{:?}", m);
    }
    eprintln!("-----------");
    eprintln!();

    eprintln!("Statistics:");
    eprintln!("-----------");
    eprintln!("Solved tasks: {}", game.count_solved_tasks());
    eprintln!("Duration: {:?}", now.elapsed());
    eprintln!("-----------");
    eprintln!();

    let file = format!("{} state.json", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    eprintln!("writing board to \"{}\"", file);
    let game_json = serde_json::to_string(&game).expect("cannot serialize game state");
    let mut file = std::fs::File::create(file).expect("cannot create file");
    file.write_all(game_json.as_bytes())
        .expect("cannot write file");
}
