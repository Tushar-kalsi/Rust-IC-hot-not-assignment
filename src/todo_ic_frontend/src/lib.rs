use leptos::*;
use leptos_meta::*;

mod app;
mod components;
mod ic_client;
mod types;

pub use app::App;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}