use crate::game::{WINDOW_H, WINDOW_W};

pub const PLAYER_X: f32 = 10.0;
pub const PLAYER_Y: f32 = 10.0;
pub const PLAYER_H: f32 = 2.0;
pub const PLAYER_W: f32 = 2.0;

pub struct Player {
  pub x: f32,
  pub y: f32,
}

impl Player {
  pub fn new(x: f32, y: f32) -> Player {
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
