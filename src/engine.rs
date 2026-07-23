use crate::game::{Game,GameState};
use crate::render::Renderer;
use crate::game::{CELL_SIZE,BOARD_X,BOARD_Y};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::{Canvas,Texture,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::mouse::MouseButton;




pub struct Engine<'a> {
    game:Game,
    render:Renderer<'a>,
    event_pump: sdl2::EventPump,
    width:usize,
    height:usize,
    mines:u8,
}

impl<'a> Engine<'a> {
    pub fn new(width:usize, height:usize,mines:u8,canvas:Canvas<Window>,event_pump: sdl2::EventPump,texture_creator:&'a TextureCreator<WindowContext>) ->Result<Self,String> {


        let mut game = Game::new(width,height,mines)?;
        let mut render = Renderer::new(canvas,&texture_creator)?;
        

        Ok({
            Self{
                game,
                render,
                event_pump,
                width,
                height,
                mines,
            }
        })
        

    }
    pub fn restart(&mut self, width: usize, height: usize, mines: u8) -> Result<(),String > {
        self.width = width;
        self.height=height;
        self.mines=mines;

        self.game=Game::new(width,height,mines)?;
        self.render.resize(width,height);
        Ok(()) 
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
            || local_x >= self.game.board.width as i32 * CELL_SIZE
            || local_y >= self.game.board.height as i32 * CELL_SIZE
        {
            return None;
        }
        let board_x = local_x; // / CELL_SIZE;
        let board_y = local_y; // / CELL_SIZE;

        Some((board_x as usize, board_y as usize))
    }

 
    pub fn handle_input(&mut self) {
        let events: Vec<_> = self.event_pump.poll_iter().collect();
        for event in events {
            match self.game.state {
                GameState::Menu => match event {
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Q => {
                            self.game.state = GameState::Quit;
                        }

                        Keycode::Space => {
                            self.game.state = GameState::Running;
                        }

                        Keycode::B => {
                            self.restart(9,9,10u8);
                        }

                        Keycode::I => {
                            self.restart(16,16,40);
                        }

                        Keycode::E => {
                            self.restart(30,16,99);
                        }

                        _ => {}
                    },

                    _ => {}
                },

                GameState::Running => {
                    match event {
                        Event::KeyDown {
                            keycode: Some(Keycode::Q),
                            ..
                        } =>{
                            self.game.state = GameState::Menu;
                        }
                        Event::MouseButtonDown {
                            mouse_btn ,
                            x,
                            y,
                            ..
                        } => {
                            //here is where we get a pixel out at x,y 
                            if let Some((board_x,board_y)) = self.screen_to_board(x,y) {
                                match mouse_btn {
                                    MouseButton::Left => {
                                        if self.game.board.reveal(board_x,board_y) {
                                            self.game.state = GameState::Lost;
                                        }
                                    }
                                    MouseButton::Right => {
                                        self.game.board.toggle_flag(board_x,board_y);
                                    } _ =>{}

                                }
                            } 
                        }
                        _ => {}
                    }
                }

                GameState::Won | GameState::Lost => match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        let width=self.width;
                        let height=self.height;
                        let mines = self.mines;
                        self.restart(width,height,mines);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => {
                        self.game.state = GameState::Menu;
                    }
                    _ => {}
                },  _ => {}
            } 
        } 
    }
  
    pub fn run(&mut self) {
        while self.game.state !=GameState::Quit {
            self.handle_input();
            self.game.update();
            self.render.draw(&self.game );
        }
    }


}

 pub fn sdl_init(width: u32, height: u32) -> Result<(Canvas<Window>, sdl2::EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window_width = BOARD_X as u32 * 2 + width* CELL_SIZE as u32;
    let window_height = BOARD_Y as u32 * 2 + height* CELL_SIZE as u32;

    let window = video_subsystem
        .window("Minesweeper", window_width, window_height )
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}
