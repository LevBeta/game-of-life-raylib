# Conway's Game of Life using Rust and the Raylib

A simple implementation of Conway's Game of Life using Rust and the Raylib library bindings.

## Description

Conway's Game of Life is a cellular automaton that follows simple rules to simulate the evolution of cells on a grid. Each cell can be in one of two states: alive or dead. The game evolves in discrete steps, or generations, based on the following rules:

- A live cell with fewer than two live neighbors dies (underpopulation).
- A live cell with two or three live neighbors survives.
- A live cell with more than three live neighbors dies (overpopulation).
- A dead cell with exactly three live neighbors becomes alive (reproduction).
 
## Controls

- R: Restarts the board with fully random cells.
- Esc: Closes window

## Instalation

 - Install and the dependencies
 - ``` git clone https://github.com/LevBeta/game-of-life-raylib ```
 - ``` cargo run ```

## Customize
You can customize the initial state of the board and other settings in the main.rs file. Adjust the ```cell_size```, ```rows```, and ```cols``` constants to change the grid and size.

## Dependencies

 - Rust 
 - Raylib library bindings for Rust
