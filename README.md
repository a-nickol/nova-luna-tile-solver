# nova-luna-tile-solver [![Build Status](https://app.travis-ci.com/anickol/nova-luna-tile-solver.svg?branch=main)](https://app.travis-ci.com/anickol/nova-luna-tile-solver)

This repository hosts a [Monte Carlo tree search] solver for the tile placing part of the [Nova Luna] board game.

The development is still in progress.

The application consists of two parts:
- `nova-luna-solver`
- `nova-luna-gui`

[Monte Carlo tree search]: https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
[Nova Luna]: https://de.wikipedia.org/wiki/Nova_Luna

## Dependencies

The `nova-luna-solver` uses Monte Carlo Tree Search to find the best solution. It depends on [MCTS] to accomplish this.

The `nova-luna-gui` uses [yew] to display the game board.

[MCTS]: https://crates.io/crates/mcts
[yew]: https://github.com/yewstack/yew

## Installation

    git clone git@github.com:a-nickol/nova-luna-tile-solver.git
    cd nova-luna-tile-solver
    cargo check

### nova-luna-gui

To run the GUI, you need to setup [trunk].

    cargo install trunk wasm-bindgen-cli
    rustup target add wasm32-unknown-unknown

[trunk]: https://trunkrs.dev/

## Configuration

To be defined.

## Usage

### nova-luna-solver

    cat resources/tiles.json | cargo run --release

    cargo run --bin nova-luna-tile-solver -- --input resources/tiles.json --statistics --playouts 10000

### nova-luna-gui

    cd nova-luna-gui
    trunk serve

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
