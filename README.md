# Minesweeper

Once again, using a classic game as a good excuse to practice Rust, this time Minesweeper.

This is the third game project, the first two being Snake and Breakout. Having learnt basic game loops, rendering, collision detection and modular code, this emphasises complex data structures, game-logic and more mature project architecture. 

## Goals

- Design and implement a 2D board.
- Become comfortable with structs, ownership, borrowing and collections.
- Algorithms on grid-based data.
- Learn more about SDL2, including mouse input and event handling.
- Load and render sprites and graphical assets.
- Proper reusable modules rather than a single source file.

## Planned Features
- Configurable board sizes.
- Random mine generation.
- left-click to reveal cells.
- Right click to flag cells.
- Automatic reveal of neighbouring empty cells.
- Win/loss detection.
- Mine counter.
- Sprite-based graphics.

## Planned structure:

src/
├── main.rs // SDL setup and main loop
├── game.rs // Overall game state
├── board.rs // Board representation and game logic
├── renderer.rs // Drawing the board and UI
├── input.rs // Mouse and keyboard input
└── assets.rs // Texture and sprite management

## Acknowledgements:

### Assets 
this project uses graphics from the LibreMines project:
https://github.com/Bollos00/LibreMines

They are utilised in accordance with the LibreMines licensing terms.

The code in this repository is my own and not derived from LibreMines. It is written independently as a Rust learning project.
