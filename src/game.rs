use crate::player::Player;
use ggez::event::{Keycode, Mod};
use ggez::*;

pub const WINDOW_W: u32 = 500;
pub const WINDOW_H: u32 = 500;
pub const ENEMIES: usize = 10;
const FPS: u32 = 100;

const INTRO_STATE: &'static str = "intro";
const VICTORY_STATE: &'static str = "victory";
const GAME_OVER_STATE: &'static str = "over";
const PLAYING_STATE: &'static str = "game";

pub struct MainState {
  player: Player,
  prize: Player,
  baddies: Vec<Player>,
  screen: &'static str,
}

impl MainState {
  pub fn new() -> GameResult<MainState> {
    let (player, prize) = crate::player::create_players();
    let baddies = crate::player::create_baddies();
    let s = MainState {
      player: player,
      prize: prize,
      baddies: baddies,
      screen: INTRO_STATE,
    };
    Ok(s)
  }

  fn start_game(&mut self, _ctx: &mut Context) {
    self.screen = PLAYING_STATE;
  }

  fn check_end_game(&mut self, prize_collision: bool, baddie_collision: bool) {
    match (prize_collision, baddie_collision) {
      (true, true) => self.end_game(GAME_OVER_STATE),
      (false, true) => self.end_game(GAME_OVER_STATE),
      (true, false) => self.end_game(VICTORY_STATE),
      (false, false) => {}
    }
  }

  fn end_game(&mut self, state: &'static str) {
    let (player, prize) = crate::player::create_players();
    self.player = player;
    self.prize = prize;
    self.screen = state;
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    while timer::check_update_time(ctx, FPS) {
      self.player.displace();
      let prize_collision: bool =
        crate::player::overlap(self.player.x, self.player.y, self.prize.x, self.prize.y);
      let mut baddie_collision: bool = false;
      for baddie in &self.baddies {
        if crate::player::overlap(self.player.x, self.player.y, baddie.x, baddie.y) {
          baddie_collision = true
        }
      }
      self.check_end_game(prize_collision, baddie_collision);
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    match self.screen {
      INTRO_STATE => crate::shapes::draw_intro(ctx),
      GAME_OVER_STATE => crate::shapes::draw_game_over(ctx),
      VICTORY_STATE => crate::shapes::draw_victory(ctx),
      PLAYING_STATE => {
        crate::shapes::draw_light(ctx, self.player.x, self.player.y);
        crate::shapes::draw_player(ctx, self.player.x, self.player.y);
        for baddie in &self.baddies {
          crate::shapes::draw_baddie(ctx, baddie, self.player.x, self.player.y);
        }
        crate::shapes::draw_prize(
          ctx,
          self.player.x,
          self.player.y,
          self.prize.x,
          self.prize.y,
        );
      }
      _ => {}
    }
    graphics::present(ctx);
    Ok(())
  }

  fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
    let playing: bool = self.screen == PLAYING_STATE;
    match (keycode, playing) {
      (Keycode::Left, true) => self.player.start("left"),
      (Keycode::Right, true) => self.player.start("right"),
      (Keycode::Up, true) => self.player.start("up"),
      (Keycode::Down, true) => self.player.start("down"),
      (Keycode::Space, false) => self.start_game(ctx),
      _ => {}
    }
  }

  fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _: Mod, _: bool) {
    let playing: bool = self.screen == PLAYING_STATE;
    match (keycode, playing) {
      (Keycode::Left, true) => self.player.stop("left"),
      (Keycode::Right, true) => self.player.stop("right"),
      (Keycode::Up, true) => self.player.stop("up"),
      (Keycode::Down, true) => self.player.stop("down"),
      _ => {}
    }
  }
}
