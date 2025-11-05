use crate::views::App;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod views;

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("RustyVpn")
                    .with_inner_size(LogicalSize::new(450.0, 650.0))
                    .with_resizable(false),
            ),
        )
        .launch(App);
}
