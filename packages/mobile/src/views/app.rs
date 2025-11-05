use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
