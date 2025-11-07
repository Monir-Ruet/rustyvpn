use crate::route::Route;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/global.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
