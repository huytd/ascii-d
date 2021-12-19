use druid::PlatformError;
use wasm_bindgen::prelude::*;

pub mod app;
pub mod consts;
pub mod data;
pub mod shapes;
pub mod tools;
pub mod widgets;

#[wasm_bindgen]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    app::start_app();
}
