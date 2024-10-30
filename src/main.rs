// #![allow(dead_code)]

use game::Game;

mod board;
mod color_schemes;
mod config;
mod debug_info;
mod game;
mod input;
mod notifications;
mod rendering;
mod rules;
mod ui;
mod utils;

#[macroquad::main("Game of Life")]
async fn main() {
    env_logger::init();

    Game::new().run().await;
}
