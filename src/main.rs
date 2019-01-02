extern crate ggez;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{DrawMode, Rect};
use ggez::*;

const TITLE: &str = "Fog of War";
const GAME_ID: &str = "1.0";
const AUTHOR: &str = "Berto";
const WINDOW_W: u32 = 500;
const WINDOW_H: u32 = 500;
const PLAYER_X: f32 = 10.0;
const PLAYER_Y: f32 = 10.0;
const PLAYER_H: f32 = 2.0;
const PLAYER_W: f32 = 2.0;

struct Player {
    x: f32,
    y: f32,
}

struct MainState {
    player: Player,
}

impl Player {
    fn new(x: f32, y: f32) -> Player {
        Player { x: x, y: y }
    }
    pub fn move_left(&mut self) {
        if self.x > 0.0 {
            self.x = self.x - 1.0;
        }
    }
    pub fn move_right(&mut self) {
        if self.x + PLAYER_W < WINDOW_W as f32 {
            self.x = self.x + 1.0;
        }
    }
    pub fn move_up(&mut self) {
        if self.y > 0.0 {
            self.y = self.y - 1.0;
        }
    }
    pub fn move_down(&mut self) {
        if self.y + PLAYER_H < WINDOW_H as f32 {
            self.y = self.y + 1.0;
        }
    }
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let player = Player::new(PLAYER_X, PLAYER_Y);
        let s = MainState { player: player };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(self.player.x, self.player.y, PLAYER_W, PLAYER_H),
        )?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Left => self.player.move_left(),
            Keycode::Right => self.player.move_right(),
            Keycode::Up => self.player.move_up(),
            Keycode::Down => self.player.move_down(),
            _ => {}
        }
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = TITLE.to_string();
    c.window_mode.height = WINDOW_H;
    c.window_mode.width = WINDOW_W;
    let ctx = &mut Context::load_from_conf(GAME_ID, AUTHOR, c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
