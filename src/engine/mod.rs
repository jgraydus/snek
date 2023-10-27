mod events;
mod geometry;
mod render;

pub use events::*;
pub use geometry::*;
pub use render::*;

use async_trait::async_trait;
use crate::constants::*;
use crate::util;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[async_trait(?Send)]
pub trait Game {
  async fn init(&self) -> Result<Box<dyn Game>, ()>;
  fn update(&mut self, key_state: &KeyState);
  fn draw(&self, renderer: &Renderer);
}

pub struct Engine {}

impl Engine {
  pub async fn start(game: impl Game + 'static) -> Result<(), String> {
    // get the canvas and ensure it's the correct size
    let canvas = util::get_canvas();
    canvas.set_height(CANVAS_HEIGHT);
    canvas.set_width(CANVAS_WIDTH);

    let renderer = Renderer::new(&canvas, JsValue::from("black"));
    let mut key_press_processor = KeyPressProcessor::new(&canvas);
    let mut game = game.init().await.unwrap();
    let mut last_frame = util::now();
    let mut accumulated_time = 0.0;

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(
      util::create_raf_closure(move |perf: f64| {
        let delta = (perf - last_frame) / 1000.0;  // perf is in milliseconds
        accumulated_time += delta;
        while accumulated_time > FRAME_LENGTH {
          let key_state = key_press_processor.process();          
          game.update(&key_state);
          accumulated_time -= FRAME_LENGTH;
        }
        last_frame = perf;
        game.draw(&renderer);
        util::request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
      })
    );

    util::request_animation_frame(g.borrow().as_ref().unwrap()).unwrap();

    Ok(())
  }
}

