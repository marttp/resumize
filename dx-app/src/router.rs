use dioxus::prelude::*;
use crate::pages::home::Home;
use crate::pages::upload::Upload;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/upload")]
    Upload {},
}