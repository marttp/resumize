use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "flex justify-around p-4 bg-vibrant-pink",
            Link {
                class: "text-white font-bold px-4 py-3 rounded-md text-md hover:shadow-lg hover:bg-light-pink",
                to: Route::Home {},
                "Generate Resume"
            }
            Link {
                class: "text-white font-bold px-4 py-3 rounded-md text-md hover:shadow-lg hover:bg-light-pink",
                to: Route::Upload {},
                "Experience Upload"
            }
        }
    }
}