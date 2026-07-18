const CELL_SIZE: i32 = 32;

const BOARD_X: i32 = 50;
const BOARD_Y: i32 = 50;

use crate::board::{Board, Cell};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

enum GameState {
    Menu,
    Running,
    Won,
    Lost,
    Quit,
}
pub struct Game {
    //SDL
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,

    //game state
    board: Board,
    mines: u8,
    //State
    state: GameState,
}

impl Game {
    fn new() -> Result<Game, String> {
        let board = Board::new(9, 9, 10);
        let (canvas, event_pump) = sdl_init(9, 9)?;

        Ok(Game {
            canvas,
            event_pump,
            board,
            mines: 10,
            state: GameState::Menu,
        })
    }
    fn board_to_screen(&self, x: usize, y: usize) -> (i32, i32) {
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

    fn handle_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match self.state {
                GameState::Menu => match event {
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Q => {
                            self.state = GameState::Quit;
                        }

                        Keycode::Space => {
                            self.state = GameState::Running;
                        }

                        Keycode::B => {
                            self.mines = 10;
                            self.board = Board::new(9, 9, 10);
                        }

                        Keycode::I => {
                            self.mines = 40;
                            self.board = Board::new(16, 16, 40);
                        }

                        Keycode::E => {
                            self.mines = 99;
                            self.board = Board::new(30, 16, 99);
                        }

                        _ => {}
                    },

                    _ => {}
                },

                GameState::Running => {}



                _ => match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        let width = self.board.width;
                        let height = self.board.height;
                        self.board = Board::new(width, height, self.mines);
                        self.state = GameState::Running;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => {
                        self.state = GameState::Menu;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn sdl_init(width: u32, height: u32) -> Result<(Canvas<Window>, sdl2::EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Minesweeper", width, height)
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}
