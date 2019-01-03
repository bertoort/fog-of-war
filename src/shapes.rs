use crate::player::{PLAYER_H, PLAYER_W};
use ggez::graphics::{DrawMode, Point2, Rect};
use ggez::*;

const LIGHT_RADIUS: f32 = 100.0;
const LIGHT_TOLERANCE: f32 = 1.0;

pub fn draw_player(ctx: &mut Context, x: f32, y: f32) {
  graphics::set_color(ctx, crate::colors::get_player()).unwrap();
  graphics::rectangle(
    ctx,
    DrawMode::Fill,
    Rect::new(
      x - (PLAYER_W / 2.0),
      y - (PLAYER_H / 2.0),
      PLAYER_W,
      PLAYER_H,
    ),
  )
  .unwrap();
}

pub fn draw_prize(ctx: &mut Context, xc: f32, yc: f32, x: f32, y: f32) {
  if crate::player::is_inside_light(LIGHT_RADIUS, xc, yc, x, y) {
    graphics::set_color(ctx, crate::colors::get_prize()).unwrap();
    graphics::rectangle(ctx, DrawMode::Fill, Rect::new(x, y, PLAYER_W, PLAYER_H)).unwrap();
  }
}

pub fn draw_light(ctx: &mut Context, x: f32, y: f32) {
  graphics::set_color(ctx, crate::colors::get_light()).unwrap();
  graphics::circle(
    ctx,
    DrawMode::Fill,
    Point2::new(x, y),
    LIGHT_RADIUS,
    LIGHT_TOLERANCE,
  )
  .unwrap();
}

pub fn draw_intro(ctx: &mut Context) {
  graphics::set_background_color(ctx, crate::colors::get_orange());
}

pub fn draw_game_over(ctx: &mut Context) {
  graphics::set_background_color(ctx, crate::colors::get_red());
}
