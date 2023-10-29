#[macro_use]
mod util;
mod constants;
mod engine;
mod snek;

use crate::engine::Engine;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        let game = snek::SnekGame::new();
        Engine::start(game).await.unwrap();
    });

    Ok(())
}

