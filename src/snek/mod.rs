mod collision;
mod game;

use async_trait::async_trait;
use crate::constants::*;
use crate::engine::{Game, KeyState, Rect, Renderer};
use crate::snek::collision::{check_collision};
use crate::snek::game::{direction, Snek};
use wasm_bindgen::prelude::*;

pub struct SnekGame {
  ready: bool,
  frame_number: u64,
  snek: Snek,
  boundary: Rect,
}

impl SnekGame {
  pub fn new() -> Self {
    Self {
      ready: false,
      frame_number: 0,
      snek: Snek::new(),
      boundary: Rect::new(250.0, 200.0, 300.0, 200.0),
    }
  }
}

#[async_trait(?Send)]
impl Game for SnekGame {
  async fn init(&self) -> Result<Box<dyn Game>, ()> {
    Ok(Box::new(Self::new()))
  }

  fn update(&mut self, key_state: &KeyState) {
    self.frame_number += 1;

    if let Some(_) = direction(key_state) {
      self.ready = true;
    }

    if !self.ready { return; }

    self.snek.update(key_state);

    let c = check_collision(&self.snek, &self.boundary);
    log!("collision: {c:?}");
  }

  fn draw(&self, renderer: &Renderer) {
    // clear the background and draw the border
    renderer.clear();
    renderer.rect(
      &Rect::new(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64),
      None,
      Some(&JsValue::from("red")),
      Some(3.0)
    );

    // draw the boundary
    renderer.rect(
      &self.boundary,
      Some(&JsValue::from("black")),
      Some(&JsValue::from("red")),
      Some(10.0));

    // draw the snek
    self.snek.draw(renderer);
  }
}

