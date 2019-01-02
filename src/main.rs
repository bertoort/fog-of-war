extern crate ggez;
use ggez::*;

mod game;
use crate::game::MainState;
use crate::game::{AUTHOR, GAME_ID, TITLE, WINDOW_H, WINDOW_W};

mod player;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = TITLE.to_string();
    c.window_mode.height = WINDOW_H;
    c.window_mode.width = WINDOW_W;
    let ctx = &mut Context::load_from_conf(GAME_ID, AUTHOR, c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
