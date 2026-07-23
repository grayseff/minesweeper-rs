use crate::game::GameState::Lost;
use crate::game::{BOARD_X, BOARD_Y, Game, GameState};

use crate::game::CELL_SIZE;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
pub struct Renderer<'a> {
    assets: Assets<'a>,
    canvas: Canvas<Window>,
    font: Font<'a, 'static>,
    texture_creator: &'a TextureCreator<WindowContext>,
}

pub struct Assets<'a> {
    hidden: Texture<'a>,
    boom: Texture<'a>,
    mine: Texture<'a>,
    flag: Texture<'a>,
    numbers: Vec<Texture<'a>>,
}
impl<'a> Assets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
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
    pub fn new(
        canvas: Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a Sdl2TtfContext,
    ) -> Result<Self, String> {
        let assets = Assets::load(&texture_creator)?;
        let font = ttf_context
            .load_font("assets/JetBrainsMono-Regular.ttf", 24)
            .map_err(|e| e.to_string())?;
        //

        Ok(Self {
            assets,
            canvas,
            font,
            texture_creator,
        })
    }
    pub fn resize(&mut self, width: usize, height: usize) -> Result<(), String> {
        let window_width = BOARD_X as u32 * 2 + width as u32 * CELL_SIZE as u32;
        let window_height = BOARD_Y as u32 * 2 + height as u32 * CELL_SIZE as u32;

        self.canvas
            .window_mut()
            .set_size(window_width, window_height)
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    // impl Game {
    pub fn draw(&mut self, game: &Game) -> Result<(), String> {
        self.canvas.clear();

        match game.state {
            GameState::Menu => self.draw_menu(game)?,
            GameState::Running => self.draw_board(game)?,
            GameState::Won => self.draw_win(game)?,
            GameState::Lost => self.draw_loss(game)?,
            _ => {}
        }

        self.canvas.present();
        Ok(())
    }

    fn draw_board(&mut self, game: &Game) -> Result<(), String> {
        for y in 0..game.board.height {
            for x in 0..game.board.width {
                self.draw_cell(game, x, y)?;
            }
        }
        Ok(())
    }
    fn draw_cell(&mut self, game: &Game, x: usize, y: usize) -> Result<(), String> {
        let (screen_x, screen_y) = game.board_to_screen(x, y);
        let dest = Rect::new(screen_x, screen_y, CELL_SIZE as u32, CELL_SIZE as u32);

        let cell = game.board.get(x, y);
        let texture = if cell.flagged {
            &self.assets.flag
        } else if game.state == Lost && cell.mine {
            if cell.revealed {
                &self.assets.boom
            } else {
                &self.assets.mine
            }
        } else if !cell.revealed {
            &self.assets.hidden
        } else {
            &self.assets.numbers[cell.adjacent as usize]
            //draw adjacent
        };
        self.canvas.copy(texture, None, dest)?;
        Ok(())
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, colour: Color) -> Result<(), String> {
        let surface = self
            .font
            .render(text)
            .blended(colour)
            .map_err(|e| e.to_string())?;

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = sdl2::rect::Rect::new(x, y, surface.width(), surface.height());

        self.canvas
            .copy(&texture, None, target)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
    fn draw_loss(&mut self, game: &Game) -> Result<(), String> {
        self.draw_board(game)?;

        let window_width = BOARD_X as u32 * 2 + game.width as u32 * CELL_SIZE as u32;
        let window_height = BOARD_Y as u32 * 2 + game.height as u32 * CELL_SIZE as u32;

        let surface = self
            .font
            .render("You Lose!")
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = BOARD_Y / 2 - surface.height() as i32 / 2;

        self.draw_text("You Lose!", x, y, Color::WHITE)?;

        let surface = self
            .font
            .render("[Space] Replay    [Q] Menu")
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = window_height as i32 - BOARD_Y / 2 - surface.height() as i32 / 2;

        self.draw_text("[Space] Replay    [Q] Menu", x, y, Color::WHITE)?;

        Ok(())
    }
    fn draw_win(&mut self, game: &Game) -> Result<(), String> {
        self.draw_board(game)?;

        let window_width = BOARD_X as u32 * 2 + game.width as u32 * CELL_SIZE as u32;
        let window_height = BOARD_Y as u32 * 2 + game.height as u32 * CELL_SIZE as u32;

        let surface = self
            .font
            .render("Congratulations! You Win!")
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = BOARD_Y / 2 - surface.height() as i32 / 2;

        self.draw_text("Congratulations! You win!", x, y, Color::WHITE)?;

        let surface = self
            .font
            .render("[Space] Replay    [Q] Menu")
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = window_height as i32 - BOARD_Y / 2 - surface.height() as i32 / 2;

        self.draw_text("[Space] Replay    [Q] Menu", x, y, Color::WHITE)?;

        Ok(())
    }

    fn draw_menu(&mut self, game: &Game) -> Result<(), String> {
        self.draw_board(game)?;

        let window_width = BOARD_X as u32 * 2 + game.width as u32 * CELL_SIZE as u32;
        let window_height = BOARD_Y as u32 * 2 + game.height as u32 * CELL_SIZE as u32;

        let surface = self
            .font
            .render("MINESWEEPER")
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = BOARD_Y / 2 - surface.height() as i32 / 2;

        self.draw_text("MINESWEEPER", x, y, Color::WHITE)?;

        let surface = self
            .font
            .render("[Space] Replay    [Q] Quit")
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = window_height as i32 - BOARD_Y / 2 - surface.height() as i32 / 2;

        self.draw_text("[Space] Replay    [Q] Quit", x, y, Color::WHITE)?;
        let line_spacing = 40;
        let centre = window_height as i32 / 2;

        let surface = self
            .font
            .render("[B]eginner 9 x 9 10 mines")
            .blended(Color::RED)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = centre - line_spacing;

        self.draw_text("[B]eginner 9 x 9  10 mines", x, y, Color::RED)?;

        let surface = self
            .font
            .render("[I]ntermediate 16x16 40mines")
            .blended(Color::RED)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = centre;

        self.draw_text("[I]ntermediate 16x16 40mines", x, y, Color::RED)?;

        let surface = self
            .font
            .render("[E]xpert 30 x 16 99mines")
            .blended(Color::RED)
            .map_err(|e| e.to_string())?;

        let x = window_width as i32 / 2 - surface.width() as i32 / 2;
        let y = centre + line_spacing;

        self.draw_text("[E]xpert 30 x 16 99mines", x, y, Color::RED)?;

        Ok(())
    }
}
// }
