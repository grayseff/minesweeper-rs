mod board;
mod game;
mod render;
mod engine;
use crate::game::{Game,GameState};
use crate::render::Renderer;
use crate::engine::{Engine,sdl_init};

fn main() -> Result<(),String> {
    let width = 9usize;
    let height = 9usize;
    let mines= 10u8;
    let ( canvas,event_pump ) = sdl_init(width as u32 ,height as u32 )?;

    let texture_creator = canvas.texture_creator();
    
    let mut engine = Engine::new(
        width,
        height,
        mines,
        canvas,
        event_pump,
        &texture_creator,
        )?;

        
   engine.run();


    Ok(())
}
