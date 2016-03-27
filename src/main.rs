mod world;
mod game;
mod entity;

use game::{Game};
use entity::player::{Player};

fn main() {
    let mut game = Game::new();
    game.start();
}
