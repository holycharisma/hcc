#![recursion_limit = "512"]

mod app;
mod emoji;
mod encryption;
mod htmx;

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

pub use encryption::recv_claims;
pub use encryption::SharedKeyring;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::home::App>();

    Ok(())
}
