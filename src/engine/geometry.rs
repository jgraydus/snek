#[derive(Clone, Debug)]
pub struct Point2d {
  pub x: f64,
  pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Rect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

impl Rect {
  pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
    Self { x, y, width, height }
  }
}

