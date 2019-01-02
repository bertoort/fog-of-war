use crate::player::Player;
use crate::player::{PLAYER_H, PLAYER_W, PLAYER_X, PLAYER_Y};
use ggez::event::{Keycode, Mod};
use ggez::graphics::{DrawMode, Rect};
use ggez::*;

pub const WINDOW_W: u32 = 500;
pub const WINDOW_H: u32 = 500;
const FPS: u32 = 60;

pub struct MainState {
  player: Player,
}

impl MainState {
  pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
    let player = Player::new(PLAYER_X, PLAYER_Y);
    let s = MainState { player: player };
    Ok(s)
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    while timer::check_update_time(ctx, FPS) {
      self.player.displace();
    }
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
      Keycode::Left => self.player.start("left"),
      Keycode::Right => self.player.start("right"),
      Keycode::Up => self.player.start("up"),
      Keycode::Down => self.player.start("down"),
      _ => {}
    }
  }
  fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
    match keycode {
      Keycode::Left => self.player.stop("left"),
      Keycode::Right => self.player.stop("right"),
      Keycode::Up => self.player.stop("up"),
      Keycode::Down => self.player.stop("down"),
      _ => {}
    }
  }
}
