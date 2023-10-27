use async_trait::async_trait;
use crate::constants::*;
use crate::engine::{Game, KeyState, Point2d, Rect, Renderer};
use crate::snek::collision::{Collision};
use crate::snek::entity::{Boundary, Direction, direction, Snek};
use crate::snek::pill::{Pill,PillType};
use rand;
use wasm_bindgen::prelude::*;

pub struct SnekGame {
  ready: bool,
  game_over: bool,
  frame_number: u64,
  snek: Snek,
  boundary: Boundary,
  pills: Vec<Pill>,
  frames_since_pill_spawn: u64,
}

impl SnekGame {
  pub fn new() -> Self {
    Self {
      ready: false,
      game_over: false,
      frame_number: 0,
      snek: Snek::new("white".to_string(),
                      60.0,
                      Point2d { x: 400.0, y: 300.0 },
                      Direction::Up),
      boundary: Boundary::new(),
      pills: Vec::new(),
      frames_since_pill_spawn: 0,
    }
  }
}

#[async_trait(?Send)]
impl Game for SnekGame {
  async fn init(&self) -> Result<Box<dyn Game>, ()> {
    Ok(Box::new(Self::new()))
  }

  fn update(&mut self, key_state: &KeyState) {
    // ---------------------------------------------------------------------
    // bookkeeping
    self.frame_number += 1;
    self.frames_since_pill_spawn += 1;
    // ready flag prevents game from starting until user provides an input
    if let Some(_) = direction(key_state) { self.ready = true; }
    if !self.ready || self.game_over { return; }

    // ---------------------------------------------------------------------
    // check for collisions
    if self.snek.colliding(&()) || self.snek.colliding(&self.boundary) {
      self.game_over = true;
    }

    // check for pill collisions
    if let Some(i) = self.snek.colliding(&self.pills) {
      log!("collided with pill {i:?}");
      let pill = self.pills.remove(i);
      match pill.pill_type {
        PillType::ExpandBoundary => { self.boundary.expand(); },
        PillType::ShortenSnek => { self.snek.shorten(0.1); },
      }
    }

    // ---------------------------------------------------------------------
    // update game state
    self.snek.update(key_state);

    // maybe spawn a pill
    if rand::random::<f64>() < 0.0001 * self.frames_since_pill_spawn as f64 {
      let Point2d { x, y } = self.boundary.random_point();
      let pill = if rand::random() {
        Pill::new(PillType::ExpandBoundary, x, y)
      } else {
        Pill::new(PillType::ShortenSnek, x, y)
      };
      self.pills.push(pill);
      self.frames_since_pill_spawn = 0;
    }
  }

  fn draw(&self, renderer: &Renderer) {
    // clear the background and draw the border
    renderer.clear();
    renderer.rect(
      &Rect::new(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64),
      None,
      Some(&JsValue::from("red")),
      Some(5.0));

    if self.game_over {
      renderer.text("GAME OVER", "red", 30, 280.0, 320.0);
      return;
    }

    // draw the boundary
    self.boundary.draw(renderer);

    // draw the snek
    self.snek.draw(renderer);

    // draw the pills
    for pill in &self.pills {
      pill.draw(renderer);
    }
  }
}

