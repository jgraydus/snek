use crate::engine::{Point2d,Renderer};
use wasm_bindgen::prelude::*;

const RADIUS: f64 = 5.0;

pub enum PillType {
  ExpandBoundary,
  ShortenSnek
}

impl PillType {
  fn to_js_value(&self) -> JsValue {
    match self {
      PillType::ExpandBoundary => JsValue::from("yellow"),
      PillType::ShortenSnek => JsValue::from("blue"),
    }
  }
}

pub struct Pill {
  pub pill_type: PillType,
  pub position: Point2d,
}

impl Pill {
  pub fn new(pill_type: PillType, x: f64, y: f64) -> Self {
    Self { pill_type, position: Point2d { x, y } }
  }

  pub fn draw(&self, renderer: &Renderer) {
    renderer.circle(&self.position, RADIUS,
                    Some(&self.pill_type.to_js_value()),
                    None,
                    None);
  }

  pub fn contains(&self, p: &Point2d) -> bool {
    let Point2d { x, y } = self.position;
    let a = x - p.x;
    let b = y - p.y;
    (a*a + b*b) < (RADIUS*RADIUS)
  }
}

