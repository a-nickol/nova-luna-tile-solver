#[macro_use]
extern crate rocket;

use nova_luna_solver::nova_luna::Tile;
use nova_luna_solver::SolverParameters;
use rocket::serde::json::Json;

#[post("/solve", data = "<tiles>")]
fn solve_board(tiles: Json<Vec<Tile>>) -> String {
    let param = SolverParameters {
        tiles: vec![],
        output_file: None,
        output_dir: None,
        print_statistics: true,
        print_moves: true,
        num_playouts: 10,
        num_threads: 1,
        debug: true,
        exploration_constant: 2.0,
    };
    let parameters = SolverParameters {
        tiles: tiles.to_vec(),
        ..param
    };
    let state = nova_luna_solver::solve(parameters);
    nova_luna_solver::game_state_as_json(&state)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![solve_board])
}
