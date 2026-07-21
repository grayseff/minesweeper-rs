mod board;
mod game;
mod render;
use crate::game::Game;


fn main() -> Result<(),String> {
    let mut game = Game::new();
    Ok(())
}
