use crate::game::{Game,GameState};
use crate::board;

use sdl2::pixels::Color;
use sdl2::render::{Canvas,Texture,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::image::{InitFlag, LoadTexture};



pub struct Assets<'a> {
    hidden: Texture<'a>,
    boom: Texture<'a>,
    mine: Texture<'a>,
    flag: Texture<'a>,
    numbers: Vec<Texture<'a>>,
}
impl<'a> Assets<'a> {
    pub fn load(
        texture_creator: &'a TextureCreator<WindowContext>,
        ) -> Result<Self, String> {
        let hidden = texture_creator.load_texture("assets/graphics/no_flag.png")?;
        let boom = texture_creator.load_texture("assets/graphics/boom.png")?;
        let mine = texture_creator.load_texture("assets/graphics/mine.png")?;
        let flag = texture_creator.load_texture("assets/graphics/flag.png")?;
        let mut numbers = Vec::new();

        for i in 0..=8 {
            let path = format!("assets/graphics/{}.png", i);
            numbers.push(texture_creator.load_texture(path)?);
        }
        Ok(Assets {
            hidden,
            boom,
            mine,
            flag,
            numbers,
        })
    }
}




impl Game {
    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.clear();

        match self.state{
            GameState::Menu => self.draw_menu()?,
            GameState::Running => self.draw_board()?,
            GameState::Won => self.draw_win()?,
            GameState::Lost => self.draw_loss(),
            _=> {}
        } 

    }
    
    fn draw_board( &self ) {
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                self.draw_cell(x,y)?;
            }
        }
    }
    fn draw_cell(&self, x:usize , y:usize ) {
        let cell = self.board.get(x,y);
        if cell.flagged {
            //draw flag 
        } else if !cell.revealed {
            // draw hidden 
        } else if cell.mine {
            //draw mine 
        } else if cell.adjacent == 0 {
            //draw blank
        } else {
            //draw adjacent 
        }
    }

}
