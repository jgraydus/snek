use futures::channel::mpsc::{channel, Receiver};
use wasm_bindgen::prelude::*;
use web_sys;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Key {
  Up, Down, Left, Right, Other
}

impl Key {
  fn from_code(code: String) -> Self {
    match code.as_str() {
      "ArrowUp" | "KeyW" => Key::Up,
      "ArrowDown" | "KeyS" => Key::Down,
      "ArrowLeft" | "KeyA" => Key::Left,
      "ArrowRight" | "KeyD" => Key::Right,
      _ => Key::Other,
    }
  }
}

#[derive(Clone, Debug)]
pub struct KeyState {
  up: bool,
  down: bool,
  left: bool,
  right: bool,
}

impl KeyState {
  fn new() -> Self {
    Self { up: false, down: false, left: false, right: false }
  }

  pub fn is_pressed(&self, key: Key) -> bool {
    match key {
      Key::Up => self.up,
      Key::Down => self.down,
      Key::Left => self.left,
      Key::Right => self.right,
      Key::Other => false,
    }
  }

  fn set_key_pressed(&mut self, key: Key, value: bool) {
    match key {
      Key::Up => self.up = value,
      Key::Down => self.down = value,
      Key::Left => self.left = value,
      Key::Right => self.right = value,
      Key::Other => {},
    }
  }
}

pub enum Event {
  KeyDown(web_sys::KeyboardEvent),
  KeyUp(web_sys::KeyboardEvent),
}

pub struct KeyPressProcessor {
  receiver: Receiver<Event>,
  key_state: KeyState,
}

impl KeyPressProcessor {
  pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
    let (s, r) = channel::<Event>(10);

    let mut s1 = s.clone();
    let keydown_handler = Closure::new(move |evt: web_sys::KeyboardEvent| {
      s1.try_send(Event::KeyDown(evt)).unwrap();
    });
    canvas.set_onkeydown(Some(keydown_handler.as_ref().unchecked_ref()));
    keydown_handler.forget();

    let mut s2 = s.clone();
    let keyup_handler = Closure::new(move |evt: web_sys::KeyboardEvent| {
      s2.try_send(Event::KeyUp(evt)).unwrap();
    });
    canvas.set_onkeyup(Some(keyup_handler.as_ref().unchecked_ref()));
    keyup_handler.forget();

    KeyPressProcessor { receiver: r, key_state: KeyState::new() }
  }

  // process all key presses received since last process and return the updated state
  pub fn process(&mut self) -> KeyState {
    while let Ok(Some(evt)) = self.receiver.try_next() {
      match evt {
        Event::KeyDown(evt) => {
          self.key_state.set_key_pressed(Key::from_code(evt.code()), true);
        },
        Event::KeyUp(evt) => {
          self.key_state.set_key_pressed(Key::from_code(evt.code()), false);
        }
      }
    }
    self.key_state.clone()
  }
}

