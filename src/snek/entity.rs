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

#[derive(Clone)]
pub struct Snek {
  color: String,
  speed: f64, // pixels per second
  path: Vec<Point2d>,
  direction: Direction,
}

impl Snek {
  pub fn new(color: String, speed: f64, position: Point2d, direction: Direction) -> Self {
    let mut path = Vec::new();
    let Point2d { x, y } = position.clone();
    match direction {
      Direction::Up    => { path.push(Point2d { x, y: y+20.0 }); }
      Direction::Down  => { path.push(Point2d { x, y: y-20.0 }); }
      Direction::Left  => { path.push(Point2d { x: x+20.0, y }); }
      Direction::Right => { path.push(Point2d { x: x-20.0, y }); }
    }
    path.push(position);
    Self { color, speed, path, direction }
  }

  pub fn path(&self) -> &Vec<Point2d> { &self.path }
  pub fn direction(&self) -> Direction { self.direction }

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
    renderer.path(&self.path, &JsValue::from(&self.color), Some(10.0));
  }

  pub fn shorten(&mut self, percentage: f64) {
    let l = length(self);
    shorten_path(&mut self.path, l*percentage);
  }

  pub fn increase_speed(&mut self, amount: f64) {
    self.speed += amount;
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

#[derive(Clone)]
pub struct AiSnek {
  snek: Snek,
  alive: bool,
  frames_since_turn: u8,
}

impl AiSnek {
  pub fn new(color: String, speed: f64, position: Point2d, direction: Direction) -> Self {
    Self {
      snek: Snek::new(color, speed, position, direction),
      alive: true,
      frames_since_turn: 0,
    }
  }

  pub fn get(&self) -> &Snek {
    &self.snek
  }

  pub fn die(&mut self) {
    self.alive = false;
  }

  pub fn draw(&self, renderer: &Renderer) {
    self.snek.draw(renderer);
  }

  pub fn update(&mut self) {
    if !self.alive { return; }

    // TODO decision to turn
    self.frames_since_turn += 1;
    if self.frames_since_turn == 50 {
      self.frames_since_turn = 0;
      let current_pos = self.snek.path[self.snek.path.len() - 1].clone();
      self.snek.path.push(current_pos);
      match self.snek.direction {
        Direction::Up | Direction::Down => {
          self.snek.direction = if rand::random() { Direction::Left } else { Direction::Right };
        },
        Direction::Left | Direction::Right => {
          self.snek.direction = if rand::random() { Direction::Up } else { Direction::Down };
        }
      }
    }

    // move the snek
    let distance = self.snek.speed * FRAME_LENGTH;
    let i = self.snek.path.len() - 1;
    // lengthen in the direction of movement
    match self.snek.direction {
      Direction::Up    => { self.snek.path.get_mut(i).unwrap().y -= distance; }
      Direction::Down  => { self.snek.path.get_mut(i).unwrap().y += distance; }
      Direction::Left  => { self.snek.path.get_mut(i).unwrap().x -= distance; }
      Direction::Right => { self.snek.path.get_mut(i).unwrap().x += distance; }
    }
    // shorten the end
    shorten_path(&mut self.snek.path, distance*0.95);
  }
}

pub struct Exit {
  rect: Rect,
}

impl Exit {
  pub fn new() -> Self {
    Self { rect: Rect::new(780.0, 260.0, 20.0, 80.0) }
  }

  pub fn draw(&self, renderer: &Renderer) {
    renderer.rect(
      &self.rect,
      Some(&JsValue::from("white")),
      None,
      None);
    renderer.text("E", "black", 16, 783.0, 278.0);
    renderer.text("X", "black", 16, 783.0, 298.0);
    renderer.text("I", "black", 16, 787.0, 318.0);
    renderer.text("T", "black", 16, 783.0, 338.0);
  }

  pub fn contains(&self, p: &Point2d) -> bool {
    self.rect.contains(p)
  }
}

