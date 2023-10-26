use crate::constants::*;
use crate::engine;
use wasm_bindgen::prelude::*;

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub enum Direction { Left, Right, Up, Down }

pub fn direction(key_state: &engine::KeyState) -> Option<Direction> {
  if key_state.is_pressed(engine::Key::Up)    { return Some(Direction::Up); }
  if key_state.is_pressed(engine::Key::Down)  { return Some(Direction::Down); }
  if key_state.is_pressed(engine::Key::Left)  { return Some(Direction::Left); }
  if key_state.is_pressed(engine::Key::Right) { return Some(Direction::Right); }
  return None;
}

pub struct Snek {
  pub speed: f64, // pixels per second
  pub path: Vec<engine::Point2d>,
  pub direction: Direction,
}

impl Snek {
  pub fn new() -> Self {
    Self {
      speed: 40.0,
      path: vec![engine::Point2d { x: 400.0, y: 350.0 },
                 engine::Point2d { x: 400.0, y: 300.0 }],
      direction: Direction::Up,
    }
  }

  pub fn update(&mut self, key_state: &engine::KeyState) {
    // handle changing directions
    if let Some(d) = direction(key_state) {
      match (d, self.direction) {
        // ignore if the keypress is the same direction or opposite direction
        (Direction::Left, Direction::Up) |
        (Direction::Left, Direction::Down) |
        (Direction::Right, Direction::Up) |
        (Direction::Right, Direction::Down) |
        (Direction::Up, Direction::Left) |
        (Direction::Up, Direction::Right) |
        (Direction::Down, Direction::Left) |
        (Direction::Down, Direction::Right) => {
          self.direction = d;
          log!("CHANGE DIRECTIONS {d:?}");
          let current_pos = self.path[self.path.len() - 1].clone();
          self.path.push(current_pos);
        },
        _ => {}
      }
    }
    // move the snek
    let distance = self.speed * FRAME_LENGTH;
    let i = self.path.len() - 1;
    // lengthen in the direction of movement
    match self.direction {
      Direction::Up    => { self.path.get_mut(i).unwrap().y -= distance; }
      Direction::Down  => { self.path.get_mut(i).unwrap().y += distance; }
      Direction::Left  => { self.path.get_mut(i).unwrap().x -= distance; }
      Direction::Right => { self.path.get_mut(i).unwrap().x += distance; }
    }
    // shorten the end
    shorten_path(&mut self.path, distance*0.9);
  }

  pub fn draw(&self, renderer: &engine::Renderer) {
    renderer.path(&self.path, &JsValue::from("red"), Some(10.0));
  }
}

fn final_segment_length(path: &Vec<engine::Point2d>) -> f64 {
  let p0 = path.get(0).unwrap();
  let p1 = path.get(1).unwrap();
  (p0.x - p1.x).abs() + (p0.y - p1.y).abs() 
}

fn shorten_path(path: &mut Vec<engine::Point2d>, amount: f64) {
  let d = final_segment_length(path);
  if d < amount {
    path.remove(0);
    shorten_path(path, amount - d);
  } else {
    let p1 = path.get(1).unwrap().clone();
    let p0 = path.get_mut(0).unwrap();
    if p0.x == p1.x {
      if p0.y < p1.y {
        p0.y += amount;
      } else {
        p0.y -= amount;
      }
    } else {
      if p0.x < p1.x {
        p0.x += amount;
      } else {
        p0.x -= amount;
      }
    }
  } 
}

