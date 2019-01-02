use crate::game::{WINDOW_H, WINDOW_W};

pub const PLAYER_X: f32 = 10.0;
pub const PLAYER_Y: f32 = 10.0;
pub const PLAYER_H: f32 = 5.0;
pub const PLAYER_W: f32 = 5.0;

pub struct Player {
  pub x: f32,
  pub y: f32,
  left: bool,
  right: bool,
  up: bool,
  down: bool,
}

impl Player {
  pub fn new(x: f32, y: f32) -> Player {
    Player {
      x: x,
      y: y,
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
  pub fn displace(&mut self) {
    self.move_left();
    self.move_right();
    self.move_up();
    self.move_down();
  }
  fn move_left(&mut self) {
    if self.left && self.x - (PLAYER_W / 2.0) > 0.0 {
      self.x = self.x - 1.0;
    }
  }
  fn move_right(&mut self) {
    if self.right && self.x + (PLAYER_W / 2.0) < WINDOW_W as f32 {
      self.x = self.x + 1.0;
    }
  }
  fn move_up(&mut self) {
    if self.up && self.y - (PLAYER_H / 2.0) > 0.0 {
      self.y = self.y - 1.0;
    }
  }
  fn move_down(&mut self) {
    if self.down && self.y + (PLAYER_H / 2.0) < WINDOW_H as f32 {
      self.y = self.y + 1.0;
    }
  }
}
