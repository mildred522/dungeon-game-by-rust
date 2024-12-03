mod monster;
mod player;
mod game;
mod music;
mod rand;

use game::Game;
use std::time::Duration;
use std::thread;

fn main() {
    let mut game = Game::new();
    game.run();
    println!("This window will close in 5 seconds...");
    thread::sleep(Duration::from_secs(5));
}