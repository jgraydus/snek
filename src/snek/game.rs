use async_trait::async_trait;
use crate::constants::*;
use crate::engine::{Game, KeyState, Point2d, Rect, Renderer};
use crate::snek::collision::{Collision};
use crate::snek::entity::{AiSnek, Boundary, Direction, direction, Exit, Snek};
use crate::snek::pill::{Pill,PillType};
use rand;
use wasm_bindgen::prelude::*;

pub struct SnekGame {
  ready: bool,
  game_over: bool,
  win: bool,
  frame_number: u64,
  exit: Exit,
  snek: Snek,
  boundary: Boundary,
  pills: Vec<Pill>,
  frames_since_pill_spawn: u64,
  enemy_sneks: Vec<AiSnek>,
}

impl SnekGame {
  pub fn new() -> Self {
    Self {
      ready: false,
      game_over: false,
      win: false,
      frame_number: 0,
      exit: Exit::new(),
      snek: Snek::new("white".to_string(),
                      60.0,
                      Point2d { x: 400.0, y: 300.0 },
                      Direction::Up),
      boundary: Boundary::new(),
      pills: Vec::new(),
      frames_since_pill_spawn: 0,
      enemy_sneks: Vec::new(),
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
    if self.snek.colliding(&self.exit) {
      self.game_over = true;
      self.win = true;
      return;
    }
    if self.snek.colliding(&()) || self.snek.colliding(&self.boundary) {
      self.game_over = true;
      return;
    }
    for enemy_snek in &self.enemy_sneks {
      if self.snek.colliding(enemy_snek) {
        self.game_over = true;
        return;
      }
    }

    // if an enemy snek hits something, it "dies", ie. stops moving
    let mut deads = Vec::new();
    for i in 0..self.enemy_sneks.len() {
      let enemy_snek = &self.enemy_sneks[i];

      if enemy_snek.colliding(&())                 // collides with self
         || enemy_snek.colliding(&self.snek)       // collides with player
         || enemy_snek.colliding(&self.boundary) { // collides with boundary
        deads.push(i);
        continue;
      }

      for j in 0..self.enemy_sneks.len() {
        if i != j && enemy_snek.colliding(&self.enemy_sneks[j]) {
          deads.push(i);
          continue;
        }
      }
    }
    for i in deads {
      self.enemy_sneks[i].die();
    }

    // check for pill collisions
    if let Some(i) = self.snek.colliding(&self.pills) {
      log!("collided with pill {i:?}");
      let pill = self.pills.remove(i);
      match pill.pill_type {
        PillType::ExpandBoundary => { self.boundary.expand(); },
        PillType::ShortenSnek => { self.snek.shorten(0.1); },
        PillType::SpawnEnemySnek => {
          self.enemy_sneks.push(AiSnek::new("red".to_string(),
                                            40.0,
                                            self.boundary.random_point(),
                                            Direction::Up));
        }
        PillType::IncreaseSpeed => { self.snek.increase_speed(5.0); }
      }
    }

    // ---------------------------------------------------------------------
    // update game state
    self.snek.update(key_state);

    for snek in &mut self.enemy_sneks {
      snek.update();
    }

    // maybe spawn a pill
    if rand::random::<f64>() < 0.0001 * self.frames_since_pill_spawn as f64 {
      let Point2d { x, y } = self.boundary.random_point();
      let n = rand::random::<u8>() % 6;
      let pill_type = match n {
        0 | 1 | 2 => PillType::ExpandBoundary,
        3 => PillType::ShortenSnek,
        4 => PillType::SpawnEnemySnek,
        5 => PillType::IncreaseSpeed,
        _ => unreachable!(),
      };
      let pill = Pill::new(pill_type, x, y);
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
      if self.win {
        renderer.text("you have won", "red", 20, 320.0, 360.0);
      }
      return;
    }

    self.boundary.draw(renderer);
    self.exit.draw(renderer);
    self.snek.draw(renderer);

    for snek in &self.enemy_sneks {
      snek.draw(renderer);
    }

    for pill in &self.pills {
      pill.draw(renderer);
    }
  }
}

