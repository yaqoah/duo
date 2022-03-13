use std::env;
use std::path;

extern crate crossbeam_channel; 
extern crate crossbeam_utils;
extern crate ab_glyph;
extern crate ggez; 
extern crate mint;
use ggez::*;

mod constants;
mod character;
mod game; 
mod draw;

use crate::game::Game;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, WINDOW_TITLE, GAME_ID, AUTHOR};

pub fn main() {
    let assets_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };
    
    let cb = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(conf::WindowSetup::default().title(WINDOW_TITLE))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(&assets_dir);

    let (ctx, ev) = cb.build().unwrap();
    let state = Game::new().unwrap();

    event::run(ctx, ev, state);
}