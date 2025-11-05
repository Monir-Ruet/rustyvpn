use crate::views::*;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/global.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/payload-generator")]
    PayloadGenerator {},
    #[route("/tunnel-type")]
    TunnelType {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
