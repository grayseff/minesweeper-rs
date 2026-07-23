pub const CELL_SIZE: i32 = 32;

pub const BOARD_X: i32 = 50;
pub const BOARD_Y: i32 = 50;

use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Running,
    Won,
    Lost,
    Quit,
}

pub struct Game {
    pub board: Board,
    pub state: GameState,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: u8) -> Result<Self, String> {
        let board = Board::new(width, height, mines);
        Ok(Game {
            board,
            state: GameState::Menu,
            width,
            height,
        })
    }
    pub fn board_to_screen(&self, x: usize, y: usize) -> (i32, i32) {
        let pixel_x = BOARD_X + x as i32 * CELL_SIZE;
        let pixel_y = BOARD_Y + y as i32 * CELL_SIZE;

        (pixel_x, pixel_y)
    }

    pub fn update(&mut self) {
        if self.state != GameState::Running {
            return;
        }
        if self.board.has_won() {
            self.state = GameState::Won;
        }
    }
}
