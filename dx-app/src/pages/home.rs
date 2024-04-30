use std::ops::Deref;
use dioxus::prelude::*;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use crate::common::DataState;
use crate::components::navbar::Navbar;
use crate::service::resume_service::get_resume_recommendation;

#[component]
pub fn Home() -> Element {
    let mut job_title = use_signal(|| "".to_string());
    let mut job_description = use_signal(|| "".to_string());
    let mut summarize = use_signal(|| "".to_string());
    let mut data_state = use_signal(|| DataState::Idle);

    let get_recommendation = move |job_title: String, job_description: String| async move {
        data_state.set(DataState::Loading);
        get_resume_recommendation(job_title.clone(), job_description.clone()).await
            .and_then(|resume| {
                summarize.set(resume);
                data_state.set(DataState::Loaded);
                Ok(())
            })
            .unwrap_or_else(|err| {
                data_state.set(DataState::Error);
            });
    };

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
                    onclick: move |event| async move {
                        if !job_description().is_empty() {
                            get_recommendation(
                                job_title.read().clone(),
                                job_description.read().clone()
                            )
                            .await;
                            job_title.set("".to_string());
                            job_description.set("".to_string());
                        }
                    },
                    "Submit"
                }
                div {
                    class: "my-2"
                }
                div {
                    class: "overflow-auto h-72",
                    p {
                        class: "font-bold text-sm",
                        "Recommendation resume"
                    }
                    match data_state.read().deref() {
                        DataState::Loading => rsx! {
                            div {
                                class: "flex items-center",
                                div {
                                    class: "mx-2 animate-spin h-10 w-10 border-4 border-blue-500 rounded-full border-t-transparent"
                                }
                                p {
                                    "Loading recommendation..."
                                }
                            }
                        },
                        DataState::Loaded => rsx! {
                            p {
                                class: "text-justify",
                                "{summarize}"
                            }
                        },
                        DataState::Error => rsx! {
                            p {
                                class: "text-red-500",
                                "Error occurred while fetching recommendation"
                            }
                        },
                        _ => rsx! {}
                    }
                }
            }
        }
    }
}