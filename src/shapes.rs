use crate::player::Player;
use crate::player::{PLAYER_H, PLAYER_W};

use ggez::graphics::{Color, DrawMode, Font, Point2, Rect, Text};
use ggez::*;

pub const LIGHT_RADIUS: f32 = 100.0;
const LIGHT_TOLERANCE: f32 = 1.0;
const INTRO_X: f32 = 50.0;
const INTRO_Y: f32 = 30.0;
const MESSAGE_SPACING: f32 = 50.0;
const INTRO_MESSAGES: [&'static str; 4] = [
  "Something's foggy.",
  "Move with arrows",
  "and find the prize.",
  "Don't die.",
];
const INTRO_START: &str = "Hit SPACE to start";
const START_Y: f32 = 400.0;
const WIN_MESSAGE: &str = "Wow, you win.";
const END_MESSAGE: &str = "Wow, you lost.";
const END_START: &str = "Hit SPACE to replay";
const FONT_PATH: &str = "/meslo-powerline.ttf";
const FONT_SIZE: u32 = 20;

fn draw_rectangle(ctx: &mut Context, color: Color, x: f32, y: f32) {
  graphics::set_color(ctx, color).unwrap();
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

pub fn draw_player(ctx: &mut Context, player: &Player) {
  draw_rectangle(ctx, crate::colors::get_player(), player.x, player.y);
}

pub fn draw_prize(ctx: &mut Context, prize: &Player) {
  draw_rectangle(ctx, crate::colors::get_prize(), prize.x, prize.y);
}

pub fn draw_baddie(ctx: &mut Context, baddie: &Player) {
  draw_rectangle(ctx, crate::colors::get_red(), baddie.x, baddie.y);
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
  graphics::set_color(ctx, crate::colors::get_orange()).unwrap();
  let font: Font = graphics::Font::new(ctx, FONT_PATH, FONT_SIZE).unwrap();
  let mut count: f32 = 0.0;
  for text in INTRO_MESSAGES.into_iter() {
    let intro: Text = graphics::Text::new(ctx, text, &font).unwrap();
    graphics::draw(ctx, &intro, Point2::new(INTRO_X, INTRO_Y + count), 0.0).unwrap();
    count = count + MESSAGE_SPACING;
  }
  let message: Text = graphics::Text::new(ctx, INTRO_START, &font).unwrap();
  graphics::draw(ctx, &message, Point2::new(INTRO_X, START_Y), 0.0).unwrap();
}

pub fn draw_victory(ctx: &mut Context) {
  graphics::set_color(ctx, crate::colors::get_player()).unwrap();
  let font: Font = graphics::Font::new(ctx, FONT_PATH, FONT_SIZE).unwrap();
  let message: Text = graphics::Text::new(ctx, WIN_MESSAGE, &font).unwrap();
  graphics::draw(ctx, &message, Point2::new(INTRO_X, INTRO_Y), 0.0).unwrap();
  let start: Text = graphics::Text::new(ctx, END_START, &font).unwrap();
  graphics::draw(ctx, &start, Point2::new(INTRO_X, START_Y), 0.0).unwrap();
}

pub fn draw_game_over(ctx: &mut Context) {
  let font: Font = graphics::Font::new(ctx, FONT_PATH, FONT_SIZE).unwrap();
  let message: Text = graphics::Text::new(ctx, END_MESSAGE, &font).unwrap();
  graphics::draw(ctx, &message, Point2::new(INTRO_X, INTRO_Y), 0.0).unwrap();
  let start: Text = graphics::Text::new(ctx, END_START, &font).unwrap();
  graphics::draw(ctx, &start, Point2::new(INTRO_X, START_Y), 0.0).unwrap();
  graphics::set_color(ctx, crate::colors::get_red()).unwrap();
  graphics::set_background_color(ctx, crate::colors::get_background());
}
