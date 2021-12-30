#![recursion_limit = "1024"]

extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

mod app;
mod pages;
mod logics;

use app::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
