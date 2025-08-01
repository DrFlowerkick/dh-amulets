pub mod amulets;
pub mod app;
pub mod context;
pub mod home;
pub mod menu;
pub mod setup;
pub mod themes;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
