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
use std::path::Path;
use std::time::Instant;

pub fn parse_string(input: String) -> Vec<Tile> {
    serde_json::from_str(&input).expect("cannot parse tiles")
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Vec<Tile> {
    let unplaced_tiles = std::fs::read_to_string(path).expect("cannot read file");
    parse_string(unplaced_tiles)
}

pub fn solve(tiles: Vec<Tile>, output_file: Option<&str>, output_dir: Option<&str>) {
    let now = Instant::now();
    let num_tiles = tiles.len();

    let state = State::with_tiles(tiles);
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

    if let Some(dir) = output_dir {
        if let Some(file) = output_file {
            let file = file.replace(
                "${datetime}",
                &Utc::now().format("%Y-%m-%d-%H:%M:%S").to_string(),
            );

            let path = Path::new(dir).join(file);
            eprintln!("writing board to \"{}\"", path.to_string_lossy());
            let game_json = serde_json::to_string(&game).expect("cannot serialize game state");
            let mut file = std::fs::File::create(path).expect("cannot create file");
            file.write_all(game_json.as_bytes())
                .expect("cannot write file");
        }
    }
}
