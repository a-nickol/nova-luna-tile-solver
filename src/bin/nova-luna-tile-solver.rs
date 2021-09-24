use clap::{crate_authors, crate_name, crate_version, App, Arg};
use std::io::Read;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tile placement solver for the 'Nova Luna' board game. \nReads the input file from stdin, if none input file is defined.")
        .arg(
            Arg::new("INPUT_FILE")
                .long("input")
                .short('i')
                .about("Read tiles from this file in JSON format")
                .takes_value(true)
        )
        .arg(
            Arg::new("OUTPUT_FILE")
                .long("output")
                .short('o')
                .default_value("state-${datetime}.json")
                .about("Write the final state of the best game board to this file.")
                .takes_value(true)
        )
        .arg(
            Arg::new("OUTPUT_DIR")
                .long("output-directory")
                .default_value(".")
                .about("Specifies the folder the final state of the board should be written to.")
                .takes_value(true)
        )
        .get_matches();

    let output_file = matches.value_of("OUTPUT_FILE");
    let output_dir = matches.value_of("OUTPUT_DIR");

    match matches.value_of("INPUT_FILE") {
        Some(path) => {
            let tiles = nova_luna_tile_solver::parse_file(path);
            nova_luna_tile_solver::solve(tiles, output_file, output_dir);
        }
        None => {
            let stdin = std::io::stdin();
            let mut input = vec![];
            stdin
                .lock()
                .read_to_end(&mut input)
                .expect("cannot read from stdin");
            let tiles = nova_luna_tile_solver::parse_string(
                String::from_utf8(input).expect("cannot read from stdin"),
            );
            nova_luna_tile_solver::solve(tiles, output_file, output_dir);
        }
    }
}
