use crate::player::Player;
use ggez::event::{Keycode, Mod};
use ggez::*;

pub const WINDOW_W: u32 = 500;
pub const WINDOW_H: u32 = 500;
const FPS: u32 = 100;

pub struct MainState {
  player: Player,
  prize: Player,
  game_over: bool,
  display_intro: bool,
}

impl MainState {
  pub fn new() -> GameResult<MainState> {
    let (player, prize) = crate::player::create_players();
    let s = MainState {
      player: player,
      prize: prize,
      game_over: true,
      display_intro: true,
    };
    Ok(s)
  }

  fn start_game(&mut self, _ctx: &mut Context) {
    self.game_over = false;
    self.display_intro = false;
  }

  fn end_game(&mut self) {
    let (player, prize) = crate::player::create_players();
    self.player = player;
    self.prize = prize;
    self.game_over = true;
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    while timer::check_update_time(ctx, FPS) {
      self.player.displace();
      let collision: bool =
        crate::player::overlap(self.player.x, self.player.y, self.prize.x, self.prize.y);
      if collision {
        self.end_game()
      }
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    if self.game_over && self.display_intro {
      crate::shapes::draw_intro(ctx)
    } else if self.game_over {
      crate::shapes::draw_victory(ctx)
    } else {
      crate::shapes::draw_light(ctx, self.player.x, self.player.y);
      crate::shapes::draw_player(ctx, self.player.x, self.player.y);
      crate::shapes::draw_prize(
        ctx,
        self.player.x,
        self.player.y,
        self.prize.x,
        self.prize.y,
      );
    }
    graphics::present(ctx);
    Ok(())
  }

  fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
    match (keycode, self.game_over) {
      (Keycode::Left, false) => self.player.start("left"),
      (Keycode::Right, false) => self.player.start("right"),
      (Keycode::Up, false) => self.player.start("up"),
      (Keycode::Down, false) => self.player.start("down"),
      (Keycode::Space, true) => self.start_game(ctx),
      _ => {}
    }
  }

  fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
    match (keycode, self.game_over) {
      (Keycode::Left, false) => self.player.stop("left"),
      (Keycode::Right, false) => self.player.stop("right"),
      (Keycode::Up, false) => self.player.stop("up"),
      (Keycode::Down, false) => self.player.stop("down"),
      _ => {}
    }
  }
}
