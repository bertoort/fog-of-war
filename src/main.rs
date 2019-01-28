#[macro_use]
extern crate chan;
extern crate ggez;
use ggez::*;

mod colors;
mod game;
mod player;
mod shapes;

use crate::game::MainState;
use crate::game::{WINDOW_H, WINDOW_W};

use std::env;
use std::path;

const TITLE: &str = "Fog of War";
const GAME_ID: &str = "1.0";
const AUTHOR: &str = "Berto";

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = TITLE.to_string();
    c.window_mode.vsync(true);
    c.window_mode.height = WINDOW_H;
    c.window_mode.width = WINDOW_W;
    let ctx = &mut Context::load_from_conf(GAME_ID, AUTHOR, c).unwrap();
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let state = &mut MainState::new().unwrap();
    graphics::set_background_color(ctx, crate::colors::get_background());
    event::run(ctx, state).unwrap();
}
