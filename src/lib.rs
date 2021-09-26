mod nova_luna;
mod solver;

use crate::nova_luna::{State, Tile};
use crate::solver::NovaLunaBoardGameMCTS;
use crate::solver::StateEvaluator;
use chrono::Utc;
use mcts::transposition_table::*;
use mcts::tree_policy::*;
use mcts::*;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct SolverParameters<'a> {
    pub tiles: Vec<Tile>,
    pub output_file: Option<&'a str>,
    pub output_dir: Option<&'a str>,
    pub print_statistics: bool,
    pub print_moves: bool,
    pub num_playouts: u32,
    pub num_threads: usize,
    pub debug: bool,
}

#[derive(Serialize)]
struct Statistics {
    solved_tasks: usize,
    duration: Duration,
}

pub fn parse_string(input: String) -> Vec<Tile> {
    serde_json::from_str(&input).expect("cannot parse tiles")
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Vec<Tile> {
    let unplaced_tiles = std::fs::read_to_string(path).expect("cannot read file");
    parse_string(unplaced_tiles)
}

pub fn solve(param: SolverParameters) {
    let now = Instant::now();
    let num_tiles = param.tiles.len();

    let state = State::with_tiles(param.tiles.clone());
    let mut mcts = MCTSManager::new(
        state.clone(),
        NovaLunaBoardGameMCTS,
        StateEvaluator,
        UCTPolicy::new(2.0),
        ApproxTable::new(1024),
    );

    eprintln!(
        "# MCTS\nDoing {} playouts with {} threads.",
        param.num_playouts, param.num_threads
    );

    mcts.playout_n_parallel(param.num_playouts, param.num_threads);

    if param.debug {
        eprintln!("# Debug Moves");
        mcts.tree().debug_moves();
    }

    let game = playout_best_moves(&param, num_tiles, state, &mut mcts);
    print_statistics(&param, &now, &game);
    output_game_state(&param, &game)
}

fn playout_best_moves(
    param: &SolverParameters,
    num_tiles: usize,
    state: State,
    mcts: &mut MCTSManager<NovaLunaBoardGameMCTS>,
) -> State {
    if param.print_moves {
        println!("# Moves:");
    }
    let mut game = state;
    for m in mcts.principal_variation(num_tiles) {
        game.make_move(&m);
        if param.print_moves {
            println!(
                "{}",
                serde_json::to_string(&m).expect("cannot serialize move")
            );
        }
    }
    game
}

fn print_statistics(param: &SolverParameters, now: &Instant, game: &State) {
    if param.print_statistics {
        println!(
            "# Statistics:\n{}",
            serde_json::to_string(&Statistics {
                solved_tasks: game.count_solved_tasks(),
                duration: now.elapsed()
            })
            .expect("cannot print statistics")
        );
    }
}

fn output_game_state(param: &SolverParameters, game: &State) {
    let game_json = serde_json::to_string(&game).expect("cannot serialize game state");

    let mut write_to_std = true;
    if let Some(dir) = param.output_dir {
        if let Some(file) = param.output_file {
            let file = file.replace(
                "${datetime}",
                &Utc::now().format("%Y-%m-%d-%H:%M:%S").to_string(),
            );

            let path = Path::new(dir).join(file);
            eprintln!("writing board to \"{}\"", path.to_string_lossy());
            let mut file = std::fs::File::create(path).expect("cannot create file");
            file.write_all(game_json.as_bytes())
                .expect("cannot write file");
            write_to_std = false;
        }
    }

    if write_to_std {
        println!("# Game board\n{}", game_json);
    }
}
