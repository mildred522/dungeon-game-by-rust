mod monster;
mod player;
mod game;
mod music;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}