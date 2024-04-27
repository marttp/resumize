use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use crate::components::navbar::Navbar;

#[component]
pub fn Home() -> Element {
    rsx! {
        Navbar {}
        div {
            h1 { "Test" }
        }
    }
}