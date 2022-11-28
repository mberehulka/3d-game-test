#[macro_use] extern crate log;

mod game;
mod logger;
mod window;
mod settings;
mod context;
mod utils;
mod camera;
mod depth_texture;

fn main() {
    game::Game::new().start()
}