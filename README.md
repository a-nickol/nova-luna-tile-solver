# nova-luna-tile-solver [![Build Status](https://app.travis-ci.com/a-nickol/nova-luna-tile-solver.svg?branch=main)](https://app.travis-ci.com/a-nickol/nova-luna-tile-solver)

This repository hosts a [monte carlo tree search] solver for the tile placing part of the [nova luna] board game.

The development is still in progress.

The application consists of two parts:

- `nova-luna-solver`
- `nova-luna-gui`

[monte carlo tree search]: https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
[nova luna]: https://de.wikipedia.org/wiki/Nova_Luna

## Dependencies

The `nova-luna-solver` is written in Rust and uses Monte Carlo Tree Search to find the best solution. It depends on [mcts] to accomplish this.

The `nova-luna-gui` uses [Angular][angular] to display the game board.

[mcts]: https://crates.io/crates/mcts
[angular]: https://angular.io/

## Installation

Prerequisite for the installation process:

- [Rust toolchain](https://rustup.rs/) installed
- [Angular cli](https://angular.io/cli) installed

Download the sources

    git clone git@github.com:a-nickol/nova-luna-tile-solver.git
    cd nova-luna-tile-solver

Build the rust backend

    cd nova-luna-solver
    cargo check
    cd ..

Build the angular frontend

    cd nova-luna-gui
    npm install
    ng build
    cd ..

## Configuration

To be defined.

## Usage

### nova-luna-solver

    cd nova-luna-solver
    cat resources/tiles.json | cargo run --release
    cargo run --bin nova-luna-tile-solver -- --input resources/tiles.json --statistics --playouts 10000
    cd ..

### nova-luna-gui

    cd nova-luna-gui
    ng serve

## How to test the software

The unit-tests of this repository can be used to test the functionality of this library.

    cargo test
    pre-commit run --all-files

## Known issues

There are currently no know issues with this library.

## Getting help

Please you the issue tracker of the github repository if you have any problems using the library.

## Getting involved

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

For more information see [CONTRIBUTING](CONTRIBUTING.md).

## License

This software library is released under version 2.0 of the Apache License.
