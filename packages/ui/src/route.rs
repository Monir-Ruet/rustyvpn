use crate::views::*;
use dioxus::prelude::*;

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
