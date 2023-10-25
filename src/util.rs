use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys;

// make printing a console log more convenient
macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

pub fn get_canvas() -> web_sys::HtmlCanvasElement {
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let canvas = document
      .get_element_by_id("snek_canvas")
      .unwrap()
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .unwrap();
  canvas.focus();
  canvas
}

pub fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) -> Result<i32,JsValue> {
  web_sys::window().unwrap()
    .request_animation_frame(callback.as_ref().unchecked_ref())
}

pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> Closure<dyn FnMut(f64)> {
  Closure::wrap(Box::new(f))
}

pub fn now() -> f64 {
  web_sys::window().unwrap().performance().unwrap().now()
}

