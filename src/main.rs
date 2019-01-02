extern crate ggez;
use ggez::*;

mod colors;
mod game;
mod player;
mod shapes;

use crate::game::MainState;
use crate::game::{WINDOW_H, WINDOW_W};

const TITLE: &str = "Fog of War";
const GAME_ID: &str = "1.0";
const AUTHOR: &str = "Berto";

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = TITLE.to_string();
    c.window_mode.height = WINDOW_H;
    c.window_mode.width = WINDOW_W;
    let ctx = &mut Context::load_from_conf(GAME_ID, AUTHOR, c).unwrap();
    graphics::set_background_color(ctx, colors::get_background());
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
