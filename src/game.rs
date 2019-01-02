use crate::player::Player;
use crate::player::{PLAYER_X, PLAYER_Y};
use ggez::event::{Keycode, Mod};
use ggez::*;

pub const WINDOW_W: u32 = 500;
pub const WINDOW_H: u32 = 500;
const FPS: u32 = 60;

pub struct MainState {
  player: Player,
  prize: Player,
}

impl MainState {
  pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
    let (random_x, random_y) = crate::player::generate_light_location();
    let player = Player::new(PLAYER_X, PLAYER_Y);
    let prize = Player::new(random_x, random_y);
    let s = MainState {
      player: player,
      prize: prize,
    };
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
    crate::shapes::draw_light(ctx, self.player.x, self.player.y);
    crate::shapes::draw_player(ctx, self.player.x, self.player.y);
    crate::shapes::draw_prize(ctx, self.prize.x, self.prize.y);
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
