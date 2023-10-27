use crate::constants::*;
use crate::engine::{Key,KeyState,Point2d,Rect,Renderer};
use rand;
use wasm_bindgen::prelude::*;

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub enum Direction { Left, Right, Up, Down }

pub fn direction(key_state: &KeyState) -> Option<Direction> {
  if key_state.is_pressed(Key::Up)    { return Some(Direction::Up); }
  if key_state.is_pressed(Key::Down)  { return Some(Direction::Down); }
  if key_state.is_pressed(Key::Left)  { return Some(Direction::Left); }
  if key_state.is_pressed(Key::Right) { return Some(Direction::Right); }
  return None;
}

pub struct Snek {
  pub speed: f64, // pixels per second
  pub path: Vec<Point2d>,
  pub direction: Direction,
}

impl Snek {
  pub fn new() -> Self {
    Self {
      speed: 60.0,
      path: vec![Point2d { x: 400.0, y: 320.0 },
                 Point2d { x: 400.0, y: 300.0 }],
      direction: Direction::Up,
    }
  }

  pub fn update(&mut self, key_state: &KeyState) {
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
          //log!("CHANGE DIRECTIONS {d:?}");
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
    shorten_path(&mut self.path, distance*0.95);
  }

  pub fn draw(&self, renderer: &Renderer) {
    renderer.path(&self.path, &JsValue::from("red"), Some(10.0));
  }

  pub fn shorten(&mut self, percentage: f64) {
    let l = length(self);
    shorten_path(&mut self.path, l*percentage);
  }
}

fn length(snek: &Snek) -> f64 {
  let mut result = 0.0;
  for segment in snek.path.windows(2) {
    let p1 = &segment[0];
    let p2 = &segment[1];
    result += (p1.x-p2.x).abs() + (p1.y-p2.y).abs();
  }
  result
}

fn final_segment_length(path: &Vec<Point2d>) -> f64 {
  let p0 = path.get(0).unwrap();
  let p1 = path.get(1).unwrap();
  (p0.x - p1.x).abs() + (p0.y - p1.y).abs() 
}

fn shorten_path(path: &mut Vec<Point2d>, amount: f64) {
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

pub struct Boundary {
  rect: Rect  
}

impl Boundary {
  pub fn new() -> Self {
    Self { rect: Rect::new(300.0, 200.0, 200.0, 200.0) }
  }

  pub fn rect(&self) -> Rect {
    self.rect.clone()
  }

  pub fn draw(&self, renderer: &Renderer) {
    renderer.rect(
      &self.rect,
      Some(&JsValue::from("black")),
      Some(&JsValue::from("red")),
      Some(10.0));
  }

  pub fn expand(&mut self) {
    let Rect { x, y, width, height } = self.rect.clone();
    self.rect = Rect::new(
      f64::max(x - 5.0, 5.0),
      f64::max(y - 5.0, 5.0),
      f64::min(width + 10.0, CANVAS_WIDTH as f64-10.0),
      f64::min(height + 10.0, CANVAS_HEIGHT as f64-10.0));
  }

  pub fn random_point(&self) -> Point2d {
    let Rect { x, y, width, height } = self.rect.clone();
    let x = x + 10.0 + (width-20.0)*rand::random::<f64>();
    let y = y + 10.0 + (height-20.0)*rand::random::<f64>();
    Point2d { x, y }
  }
}

