use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use crate::components::navbar::Navbar;

#[component]
pub fn Home() -> Element {
    let mut job_title = use_signal(|| "".to_string());
    let mut job_description = use_signal(|| "".to_string());
    let mut summarize = use_signal(|| r#""#.to_string());

    rsx! {
        div {
            class: "flex flex-col h-screen",
            Navbar {}
            div {
                class: "flex flex-col p-4",
                div {
                    label {
                        r#for: "job_title",
                        class: "block mb-2 text-sm font-bold text-gray-900",
                        "Job title"
                    }
                    input {
                        value: "{job_title}",
                        class: "block p-2.5 w-full text-sm rounded-lg border bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
                        oninput: move |event| job_title.set(event.value())
                    }
                }
                div {
                    class: "my-2",
                    label {
                        r#for: "message",
                        class: "block mb-2 text-sm font-bold text-gray-900",
                        "Job description of your dream job"
                    }
                    textarea {
                        id: "message",
                        class: "block p-2.5 w-full text-sm rounded-lg border bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",
                        placeholder: "Place job description here...",
                        rows: 6,
                        value: "{job_description}",
                        oninput: move |event| job_description.set(event.value())
                    }
                }
                button {
                    r#type: "button",
                    class: "container my-2 px-3 py-2 text-sm font-medium text-center bg-vibrant-pink hover:bg-light-pink rounded-lg text-white transition-colors duration-200",  // Add transition for smooth color change
                    onclick: move |event| {
                        if !job_description().is_empty() {
                            summarize.set(job_description.to_string().clone());
                            job_description.set("".to_string());
                        }
                    },
                    "Submit"
                }
                hr {
                    class: "my-2"
                }
                div {
                    class: "overflow-auto p-4 my-2 h-72",
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
    }
}