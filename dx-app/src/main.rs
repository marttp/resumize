#![allow(non_snake_case)]

mod components;
mod pages;
mod router;

use dioxus::prelude::*;
use log::LevelFilter;
use crate::router::Route;

// Not support window
fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "bg-pale-yellow h-screen",
            Router::<Route> {}
        }
    }
}


