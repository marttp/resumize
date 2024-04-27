use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use crate::components::navbar::Navbar;

#[component]
pub fn Home() -> Element {
    let mut job_description = use_signal(|| "".to_string());
    let mut summarize = use_signal(|| "".to_string());
    rsx! {
        Navbar {}
        div {
            class: "p-4",
            label {
                r#for: "message",
                class: "block mb-2 text-sm font-bold text-gray-900",
                "Job description of your dream job"
            }
            textarea {
                id: "message",
                class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                placeholder: "Place job description here...",
                rows: 6,
                value: "{job_description}",
                oninput: move |event| job_description.set(event.value())
            }
            button {
                r#type: "button",
                class: "container my-2 px-3 py-2 text-sm font-medium text-center bg-vibrant-pink hover:bg-light-pink rounded-lg text-white",
                onclick: move |event| {
                    summarize.set(job_description.to_string().clone());
                    job_description.set("".to_string()); // Reset form
                },
                "Submit"
            }
            hr {
                class: "my-2"
            }
            p {
                class: "font-bold text-sm",
                "Recommendation resume"
            }
            p {
                class: "text-justify",
                "{summarize}"
            }
        }
    }
}