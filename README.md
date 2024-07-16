# Conway's Game of Life

An implementation of the Conway's Game of Life in Rust

## About

The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the Game of Life by creating an initial configuration and observing how it evolves. It is Turing complete and can simulate a universal constructor or any other Turing machine.

### Rules

The universe of the Game of Life is an infinite, two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, live or dead (or populated and unpopulated, respectively). Every cell interacts with its eight neighbors, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:

-   Any live cell with fewer than two live neighbours dies, as if by underpopulation.
-   Any live cell with two or three live neighbours lives on to the next generation.
-   Any live cell with more than three live neighbours dies, as if by overpopulation.
-   Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## How to play

Follow this steps:

1. Clone the repo: `git clone https://github.com/MarcosNicolau/conways-game-of-life.git`
2. Run it with cargo: `cargo run`
3. Enjoy it!

## Gameplay

https://github.com/MarcosNicolau/conways-game-of-life/assets/76252340/7af97424-f0da-4f3e-8a2d-6ff0f7052718


