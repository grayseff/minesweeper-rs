use crate::game::GameState::Lost;
use crate::game::{Game,GameState,CELL_SIZE};
use crate::board;

use sdl2::pixels::Color;
use sdl2::render::{Canvas,Texture,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;


pub struct Renderer<'a> {
    assets:Assets<'a>,
    canvas:Canvas<Window>,
}


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

impl<'a> Renderer<'a> {
    pub fn new(canvas: Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self,String> {
        let assets = Assets::load(&texture_creator)?;

        Ok(Self{
            assets,
            canvas,
        })
    }



// impl Game {
    pub fn draw(&mut self,game: &Game) -> Result<(), String> {
        self.canvas.clear();

        match game.state{
            // GameState::Menu => self.draw_menu()?,
            GameState::Running => self.draw_board(game)?,
            // GameState::Won => self.draw_win()?,
            // GameState::Lost => self.draw_loss(),
            _=> {}
        }

        self.canvas.present();
        Ok(())

    }
    
    fn draw_board( &mut self , game: &Game ) -> Result<(),String>{
        for y in 0..game.board.height {
            for x in 0..game.board.width {
                self.draw_cell(game,x,y)?;
            }
        }
        Ok(())
    }
    fn draw_cell(&mut self, game: &Game, x:usize , y:usize ) ->Result<(),String> {
        let ( screen_x,screen_y ) = game.board_to_screen(x,y);
        let dest = Rect::new(
            screen_x,
            screen_y,
            CELL_SIZE as u32,
            CELL_SIZE as u32,
            );


        let cell = game.board.get(x,y);
        let texture = if cell.flagged {
            &self.assets.flag 
        } else if game.state == Lost && cell.mine {
            if cell.revealed {
                &self.assets.boom
            } else {
                &self.assets.mine
            }
        }
        else if !cell.revealed {
            &self.assets.hidden
        }  else {
           &self.assets.numbers[cell.adjacent as usize] 
            //draw adjacent 
        };
        self.canvas.copy(texture,None,dest)?;
    Ok(())
    }
    fn draw_loss(&mut self , game: &Game) -> Result<(),String> {
        self.draw_board(game)?;

        Ok(())
    }

}
// }
