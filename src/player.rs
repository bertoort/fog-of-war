use crate::game::{WINDOW_H, WINDOW_W};
use rand::Rng;
use std::thread;

use crate::game::ENEMIES;
use crate::shapes::LIGHT_RADIUS;

pub const PLAYER_X: f32 = 10.0;
pub const PLAYER_Y: f32 = 10.0;
pub const PLAYER_H: f32 = 5.0;
pub const PLAYER_W: f32 = 5.0;
pub static mut READY_TO_BOOST: bool = true;
static mut BOOSTING: bool = false;
const START_SPEED: f32 = 50.0;
const MAX_SPEED: f32 = 120.0;
const SPEED: f32 = 60.0;
const ACCELERATION: f32 = 0.1;
const BOOST_TIME: f32 = 200.0;
const BOOST_COOLDOWN: f32 = 1000.0;
const BOOST_ACCELERATION: f32 = 0.5;

pub struct Player {
  pub x: f32,
  pub y: f32,
  speed: f32,
  left: bool,
  right: bool,
  up: bool,
  down: bool,
}

pub struct Baddie {
  pub x: f32,
  pub y: f32,
  speed: f32,
}

impl Player {
  pub fn new(x: f32, y: f32) -> Player {
    Player {
      x: x,
      y: y,
      speed: START_SPEED,
      left: false,
      right: false,
      up: false,
      down: false,
    }
  }
  pub fn start(&mut self, direction: &str) {
    match direction {
      "left" => self.left = true,
      "right" => self.right = true,
      "up" => self.up = true,
      "down" => self.down = true,
      _ => {}
    }
  }
  pub fn stop(&mut self, direction: &str) {
    match direction {
      "left" => self.left = false,
      "right" => self.right = false,
      "up" => self.up = false,
      "down" => self.down = false,
      _ => {}
    }
  }
  pub fn displace(&mut self, delta: f32) {
    let mut movement: f32 = SPEED * delta;
    self.speed = movement;
    unsafe {
      if BOOSTING {
        movement += BOOST_ACCELERATION;
      }
    }
    self.move_left(movement);
    self.move_right(movement);
    self.move_up(movement);
    self.move_down(movement);
  }
  fn move_left(&mut self, movement: f32) {
    if self.left && self.x - (PLAYER_W / 2.0) > 0.0 {
      self.x -= movement;
    }
  }
  fn move_right(&mut self, movement: f32) {
    if self.right && self.x + (PLAYER_W / 2.0) < WINDOW_W as f32 {
      self.x += movement;
    }
  }
  fn move_up(&mut self, movement: f32) {
    if self.up && self.y - (PLAYER_H / 2.0) > 0.0 {
      self.y -= movement;
    }
  }
  fn move_down(&mut self, movement: f32) {
    if self.down && self.y + (PLAYER_H / 2.0) < WINDOW_H as f32 {
      self.y += movement;
    }
  }
  pub fn boost(&mut self) {
    unsafe {
      if READY_TO_BOOST {
        BOOSTING = true;
        READY_TO_BOOST = false;
        let boost_timeout = chan::tick_ms(BOOST_TIME as u32);
        let reset_timeout = chan::tick_ms((BOOST_TIME + BOOST_COOLDOWN) as u32);
        thread::spawn(move || loop {
          chan_select! {
              boost_timeout.recv() => {
                BOOSTING = false;
              },
              reset_timeout.recv() => {
                READY_TO_BOOST = true;
                break;
              },
          }
        });
      }
    }
  }
}

impl Baddie {
  pub fn new(x: f32, y: f32) -> Baddie {
    Baddie {
      x: x,
      y: y,
      speed: START_SPEED,
    }
  }
  pub fn attack(&mut self, enemy: &Player, delta: f32) {
    let left: bool = self.x < enemy.x;
    let right: bool = self.x > enemy.x;
    let top: bool = self.y < enemy.y;
    let bottom: bool = self.y > enemy.y;
    let mut movement: f32 = self.speed * delta;
    if left && top || left && bottom || right && top || right && bottom {
      movement /= 2.0;
    }
    if left {
      self.x += movement
    }
    if right {
      self.x -= movement
    }
    if top {
      self.y += movement
    }
    if bottom {
      self.y -= movement
    }
    if self.speed < MAX_SPEED {
      self.speed += ACCELERATION
    }
  }
  pub fn relax(&mut self) {
    self.speed = START_SPEED
  }
}

pub fn create_players() -> (Player, Player) {
  let (random_x, random_y) = generate_location();
  let player = Player::new(PLAYER_X, PLAYER_Y);
  let prize = Player::new(random_x, random_y);
  return (player, prize);
}

pub fn generate_location() -> (f32, f32) {
  let mut rng = rand::thread_rng();
  let x = rng.gen_range(0, WINDOW_W) as f32;
  let y = rng.gen_range(0, WINDOW_H) as f32;
  if is_inside_light(LIGHT_RADIUS, x, y, PLAYER_X, PLAYER_Y) {
    return generate_location();
  }
  return (x, y);
}

pub fn is_inside_light(r: f32, xc: f32, yc: f32, x: f32, y: f32) -> bool {
  return ((xc - x).abs().powf(2.0) + (yc - y).abs().powf(2.0)).sqrt() < r;
}

pub fn overlap(x: f32, y: f32, x2: f32, y2: f32) -> bool {
  let overlap_left: bool = x + PLAYER_W > x2 && x + PLAYER_W < x2 + PLAYER_W;
  let overlap_right: bool = x - PLAYER_W < x2 && x - PLAYER_H > x2 - PLAYER_W;
  let overlap_top: bool = y < y2 + PLAYER_H && y - PLAYER_H > y2 - PLAYER_H;
  let overlap_bottom: bool = y + PLAYER_H > y2 && y + PLAYER_H < y2 + PLAYER_H;
  return (overlap_left && overlap_top)
    || (overlap_left && overlap_bottom)
    || (overlap_right && overlap_top)
    || (overlap_right && overlap_bottom);
}

pub fn create_baddies() -> Vec<Baddie> {
  let mut index: usize = 0;
  let mut baddies: Vec<Baddie> = vec![];
  while index < ENEMIES {
    let (x, y) = generate_location();
    baddies.push(Baddie::new(x, y));
    index += 1;
  }
  return baddies;
}
