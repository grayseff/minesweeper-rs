pub const CELL_SIZE: i32 = 32;

pub const BOARD_X: i32 = 50;
pub const BOARD_Y: i32 = 50;

use crate::board::{Board, Cell};


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum GameState {
    Menu,
    Running,
    Won,
    Lost,
    Quit,
}

pub struct Game {
    //SDL
    //pub canvas: Canvas<Window>,
    // pub assets: Assets<'a>
    //game state
    pub board: Board,
    mines: u8,
    //State
    pub state: GameState,
}

impl    Game {
    pub fn new(width:usize,height:usize,mines:u8) -> Result<Self, String> {
        let board = Board::new(width, height, mines);
        Ok(Game {
            board,
            mines: mines,
            state: GameState::Menu,
        })
    }
    pub fn board_to_screen(&self, x: usize, y: usize) -> (i32, i32) {
        let pixel_x = BOARD_X + x as i32 * CELL_SIZE;
        let pixel_y = BOARD_Y + y as i32 * CELL_SIZE;

        (pixel_x, pixel_y)
    }
    fn screen_to_board(&self, mouse_x: i32, mouse_y: i32) -> Option<(usize, usize)> {
        let local_x = (mouse_x - BOARD_X) / CELL_SIZE;
        let local_y = (mouse_y - BOARD_Y) / CELL_SIZE;
        if local_x < 0
            || local_y < 0
            || local_x >= self.board.width as i32 * CELL_SIZE
            || local_y >= self.board.height as i32 * CELL_SIZE
        {
            return None;
        }
        let board_x = local_x / CELL_SIZE;
        let board_y = local_y / CELL_SIZE;

        Some((board_x as usize, board_y as usize))
    }

    pub fn update(&mut self) {
       if self.state != GameState::Running {
           return;
       } 
       if self.board.has_won(){
           self.state=GameState::Won;
       }
    }
}
