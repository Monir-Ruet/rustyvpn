use dioxus::prelude::*;
use views::{App, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus::launch(App);
}
