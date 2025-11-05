use dioxus::prelude::*;

use crate::views::app::Route;

const PAYLOAD_GENERATE_ICON: Asset = asset!("/assets/icons/code.png");

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-black/90 text-gray-100 flex flex-col items-center justify-center p-6",

            div { class: "text-4xl font-extrabold mb-5 tracking-wide text-blue-400 drop-shadow-lg",
                span { class: "text-red-500", "Rusty" }
                "VPN"
            }

            div { class: "w-full max-w-md bg-black/10 backdrop-blur-md p-6 rounded-2xl shadow-2xl border border-gray-700 space-y-6 transition-all",

                div { class: "flex items-center justify-between",
                    span { class: " text-sm", "Connection Status" }
                    span { class: "px-3 py-1 text-sm rounded-full bg-red-600/90 shadow-sm",
                        "Disconnected"
                    }
                }

                div { class: "space-y-2 text-gray-300 text-sm",
                    div { class: "flex justify-between items-center",
                        span { "SSH Host:" }
                        span { class: "text-gray-100 font-mono", "vpn.example.com" }
                    }
                    div { class: "flex justify-between items-center",
                        span { "Port:" }
                        span { class: "text-gray-100 font-mono", "22" }
                    }
                    div { class: "flex justify-between items-center",
                        span { "Username:" }
                        span { class: "text-gray-100 font-mono", "root" }
                    }
                }

                div { class: "space-y-2 relative",
                    div { class: "flex justify-between items-center",
                        label { class: "text-gray-300 text-sm font-medium", "HTTP Payload" }
                        Link { to: Route::PayloadGenerator {},
                            button { class: "text-sm px-3 py-1 rounded-md bg-gray-700 hover:bg-gray-600 text-gray-200 transition-colors",
                                img {
                                    class: "w-4 h-4",
                                    src: PAYLOAD_GENERATE_ICON,
                                }

                            }
                        }
                    }

                    div { class: "mt-2 relative",
                        textarea {
                            class: "w-full h-20 bg-black/1 border-1 border-gray-700 rounded-lg p-3 text-gray-100 max-h-24 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm placeholder-gray-500",
                            placeholder: "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n",
                        }
                    }
                }

                div { class: "flex justify-center",
                    button { class: "w-full py-3 rounded-md bg-smoke/20 active:scale-95 transition-all duration-200 font-semibold text-md shadow-md shadow-blue-900/99 border-1 border-blue-500",
                        "Connect via SSH"
                    }
                }

                div { class: "border-t border-gray-800 pt-4 space-y-2 text-sm text-gray-400",
                    div { class: "flex justify-between items-center",
                        span { "Public IP:" }
                        span { class: "text-gray-200 font-mono", "Not connected" }
                    }
                    div { class: "flex justify-between items-center",
                        span { "Server Region:" }
                        span { class: "text-gray-200 font-mono", "-" }
                    }
                }
            }

            div { class: "mt-10 text-gray-600 text-xs tracking-wide",
                "SSH-based VPN | Secure Tunnel over Port 22"
            }
        }
    }
}
