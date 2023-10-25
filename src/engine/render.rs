use crate::engine::geometry::*;
use wasm_bindgen::prelude::*;
use web_sys;

// wrapper around CanvasRenderingContext2d to simplify drawing code
pub struct Renderer {
  // a rect that covers the canvas
  bounds: Rect,
  // when the clear method is called, this color is drawn to the entire canvas
  bg_color: JsValue,
  // rendering context for the canvas
  cxt: web_sys::CanvasRenderingContext2d,
}

impl Renderer {
  pub fn new(
    canvas: &web_sys::HtmlCanvasElement,
    bg_color: JsValue,
  ) -> Self {
    let bounds = Rect { x: 0.0, y: 0.0, width: canvas.width() as f64, height: canvas.height() as f64 };
    // get the rendering context
    let cxt = canvas
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    Self { bounds, bg_color, cxt }
  }

  // fill the entire canvas with the bg_color value
  pub fn clear(&self) {
    self.rect(&self.bounds, Some(&self.bg_color), None, None);
  }

  // draw a rectangle onto the canvas
  pub fn rect(
    &self,
    rect: &Rect,
    fill_style: Option<&JsValue>,   // only fills if fill_style is provided
    stroke_style: Option<&JsValue>, // only strokes if stroke_style is provided
    line_width: Option<f64>         // optional. defaults to 1.0 line width if not provided
  ) {
    let Rect { x, y, width, height } = rect.clone();
    self.cxt.begin_path();
    self.cxt.rect(x, y, width, height);
    if let Some(fill_style) = fill_style {
      self.cxt.set_fill_style(fill_style);
      self.cxt.fill();
    }
    if let Some(stroke_style) = stroke_style {
      if let Some(line_width) = line_width {
        self.cxt.set_line_width(line_width);
      } else {
        self.cxt.set_line_width(1.0);
      }
      self.cxt.set_stroke_style(stroke_style);
      self.cxt.stroke();
    }
  }

  // draw a path onto the canvas
  pub fn path(
    &self,
    points: &Vec<Point2d>,
    stroke_style: &JsValue,
    line_width: Option<f64>   // optional. defaults to 1.0 if not provided
  ) {
    if points.is_empty() {
      return;
    }
    self.cxt.begin_path();
    let Point2d { x, y } = points[0];
    self.cxt.move_to(x, y);
    for Point2d { x, y } in &points[1..] {
      self.cxt.line_to(*x, *y);
    }
    if let Some(line_width) = line_width {
      self.cxt.set_line_width(line_width);
    } else {
      self.cxt.set_line_width(1.0);
    }
    self.cxt.set_stroke_style(stroke_style);
    self.cxt.stroke();
  }
}

