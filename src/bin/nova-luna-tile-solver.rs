use clap::{crate_authors, crate_name, crate_version, App, Arg};
use nova_luna_tile_solver::SolverParameters;
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
            Arg::new("PRINT_STATISTICS")
                .long("statistics")
                .about("Activates the output of statistics.")
        )
        .arg(
        Arg::new("PRINT_MOVES")
            .long("moves")
            .about("Activates the output of the best moves.")
        )
        .arg(
            Arg::new("OUTPUT_FILE")
                .long("output")
                .short('o')
                .about("Write the final state of the best game board to this file. \"${datetime}\" will be replaced with current date and time.")
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
    let print_statistics = matches.is_present("PRINT_STATISTICS");
    let print_moves = matches.is_present("PRINT_MOVES");

    let param = SolverParameters {
        tiles: vec![],
        output_file,
        output_dir,
        print_statistics,
        print_moves,
    };

    let tiles = match matches.value_of("INPUT_FILE") {
        Some(path) => nova_luna_tile_solver::parse_file(path),
        None => nova_luna_tile_solver::parse_string(read_from_stdin()),
    };

    nova_luna_tile_solver::solve(SolverParameters { tiles, ..param });
}

fn read_from_stdin() -> String {
    let stdin = std::io::stdin();
    let mut input = vec![];
    stdin
        .lock()
        .read_to_end(&mut input)
        .expect("cannot read from stdin");
    String::from_utf8(input).expect("cannot read from stdin")
}
