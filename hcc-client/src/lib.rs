#![recursion_limit = "512"]

mod app;
mod emoji;
mod encryption;
mod hooks;
mod htmx;
mod media_renderer;

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

pub use encryption::recv_claims;
pub use encryption::SharedKeyring;

pub use media_renderer::render_media_node;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::home::App>();

    Ok(())
}
