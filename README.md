# Tic Tac Toe in Rust

A simple command-line Tic Tac Toe game implemented in Rust. This project demonstrates the basics of Rust programming, including handling user input, managing game states, and implementing a classic game in a straightforward and interactive way.

## Features

- **Interactive Gameplay**: Play Tic Tac Toe directly in the terminal.
- **Two-Player Mode**: Play with a friend in turn-based fashion.
- **Game State Management**: The program checks for winning conditions and handles draws effectively.
- **Error Handling**: Handles invalid moves, preventing overwriting of cells.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (ensure it's installed by running `rustc --version`)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/agastya3636/tic_tac_toe_in_rust.git
   cd tic_tac_toe_in_rust
   ```

2. Run the game:
   ```bash
   cargo run
   ```

## How to Play

- Players take turns entering the position (row and column) where they want to place their mark (X or O).
- The game checks for a winner or a draw after each move and displays the board.

## Project Structure

- **src/main.rs**: The main game logic and functions for displaying the board, handling turns, and checking win conditions.

## Future Improvements

- [ ] Add AI for single-player mode.
- [ ] Create a graphical version using a Rust GUI library.
- [ ] Implement a scoring system.
