use leptos::*;
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod ic_client;
mod types;

pub use app::App;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}