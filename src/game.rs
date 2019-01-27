use crate::player::{is_inside_light, Baddie, Player};
use crate::shapes::LIGHT_RADIUS;

use ggez::event::{Keycode, Mod};
use ggez::*;

pub const WINDOW_W: u32 = 500;
pub const WINDOW_H: u32 = 500;
pub const ENEMIES: usize = 50;

const INTRO_STATE: &'static str = "intro";
const VICTORY_STATE: &'static str = "victory";
const GAME_OVER_STATE: &'static str = "over";
const PLAYING_STATE: &'static str = "game";

pub struct MainState {
  player: Player,
  prize: Player,
  baddies: Vec<Baddie>,
  screen: &'static str,
  delta: f32,
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
      delta: 0.0,
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
    self.baddies = crate::player::create_baddies();
    self.player = player;
    self.prize = prize;
    self.screen = state;
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.delta = timer::duration_to_f64(timer::get_average_delta(ctx)) as f32;
    self.player.displace(self.delta);
    let prize_collision: bool =
      crate::player::overlap(self.player.x, self.player.y, self.prize.x, self.prize.y);
    let mut baddie_collision: bool = false;
    let mut index: usize = 0;
    while index < self.baddies.len() {
      if crate::player::overlap(
        self.player.x,
        self.player.y,
        self.baddies[index].x,
        self.baddies[index].y,
      ) {
        baddie_collision = true
      }
      index += 1;
    }
    self.check_end_game(prize_collision, baddie_collision);
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
        crate::shapes::draw_player(ctx, &self.player);
        let mut index: usize = 0;
        while index < self.baddies.len() {
          if is_inside_light(
            LIGHT_RADIUS,
            self.player.x,
            self.player.y,
            self.baddies[index].x,
            self.baddies[index].y,
          ) {
            self.baddies[index].attack(&self.player, self.delta);
            crate::shapes::draw_baddie(ctx, &self.baddies[index]);
          } else {
            self.baddies[index].relax();
          }
          index += 1;
        }
        if is_inside_light(
          LIGHT_RADIUS,
          self.player.x,
          self.player.y,
          self.prize.x,
          self.prize.y,
        ) {
          crate::shapes::draw_prize(ctx, &self.prize);
        }
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
      (Keycode::Space, true) => self.player.boost(),
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
