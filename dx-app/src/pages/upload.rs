use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::events::FormEvent;
use dioxus::hooks::use_signal;
use dioxus::html::{FileEngine, HasFileData};
use crate::components::navbar::Navbar;

#[derive(Debug)]
struct UploadedFile {
    name: String,
    contents: Vec<u8>,
}

#[component]
pub fn Upload() -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);
    let mut hovered = use_signal(|| false);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            println!("{}", file_name.clone());
            if let Some(contents) = file_engine.read_file(file_name).await {
                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                    contents,
                });
                println!("{:?}", files_uploaded.get(0).unwrap().name.clone());
                println!("{:?}", files_uploaded.get(0).unwrap().contents.clone());
            }
        }
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    rsx! {
        Navbar {}
        div {
            class: "p-4",
            div {
                class: "flex flex-col",
                label { r#for: "filereader", "Drop your experience in json format" }
                input {
                    name: "filereader",
                    r#type: "file",
                    accept: ".json",
                    multiple: false,
                    directory: false,
                    required: true,
                    class: "my-4 display-none",
                    onchange: upload_files
                }
                div {
                    id: "drop-zone",
                    class: "block p-2.5 w-full text-sm rounded-lg border",
                    prevent_default: "ondragover ondrop",
                    background_color: if hovered() { "lightblue" } else { "lightgray" },
                    ondragover: move |_| hovered.set(true),
                    ondragleave: move |_| hovered.set(false),
                    ondrop: move |evt| async move {
                        hovered.set(false);
                        if let Some(file_engine) = evt.files() {
                            read_files(file_engine).await;
                        }
                    },
                    "Drop files here"
                }
            }
            if !files_uploaded.is_empty() {
                p {
                    class: "my-4",
                    "Your experience has uploaded!"
                }
            }
        }
    }
}