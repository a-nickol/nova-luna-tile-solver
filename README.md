# nova-luna-tile-solver [![Build Status](https://app.travis-ci.com/anickol/nova-luna-tile-solver.svg?branch=main)](https://app.travis-ci.com/anickol/nova-luna-tile-solver)

This repository hosts a [Monte Carlo tree search] solver for the tile placing part of the [Nova Luna] board game.

The development is still in progress.

There are to parts for the application, the backend and the frontend. The backend is written in Rust. For the frontend I am still evaluating frameworks.

[Monte Carlo tree search]: https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
[Nova Luna]: https://de.wikipedia.org/wiki/Nova_Luna

## Dependencies

The solver uses Monte Carlo Tree Search to find the best solution. It depends on [MCTS](https://crates.io/crates/mcts) to accomplish this.

## Installation

To be defined.

## Configuration

To be defined.

## Usage

    cat resources/tiles.json | cargo run --release

    cargo run --release -- --input resources/tiles.json

## How to test the software

The unit-tests of this repository can be used to test the functionality of this library.

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
