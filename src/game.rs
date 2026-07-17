use crate::board::{Board,Cell};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

enum GameState {
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

    //State 
    running: GameState,
}

impl Game{
    fn new(width:usize, height:usize, num_mines:u8) -> Result<Game,String>{
        let board = Board::new(width,height,num_mines);
        let (canvas,event_pump) = sdl_init(width as u32, height as u32)?;

        Ok(Game {
            canvas,
            event_pump,
            board,
            state: GameState::Running,
        })
    }
        
}
fn sdl_init(width: u32, height: u32) -> Result<(Canvas<Window>, EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Minesweeper", width, height)
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}
