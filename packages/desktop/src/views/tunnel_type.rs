use dioxus::prelude::*;

#[component]
pub fn TunnelType() -> Element {
    rsx! {
        div { class: "min-h-screen bg-black/92 flex flex-col p-5 gap-4",
            div { class: "w-full max-w-md bg-black/10 backdrop-blur-md p-2 rounded-md shadow-2xl border border-gray-700 ",
                div { class: "flex flex-col gap-1  p-4 rounded-lg",
                    div { class: "text-sm text-white/80 m-0", "Selected Tunnel Type" }
                    div { class: "text-sm text-white/80 m-0", "SSL/TLS -> SSH" }
                    div { class: "text-xs text-white/80 m-0 bg-blue-500 w-fit rounded-md",
                        "TCP/(TLS)"
                    }
                }
            }

            div { class: "w-full max-w-md bg-black/10 backdrop-blur-md p-2 rounded-md shadow-2xl border border-gray-700 space-y-6 text-white/70",

                div {
                    label { "Tunnel Type" }
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "normal",
                                class: "w-5 h-5 text-red-600 bg-gray-100 border-gray-300 focus:ring-red-500 dark:focus:ring-red-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                checked: true,
                            }
                            label { r#for: "normal", "Normal" }
                        }

                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "split",
                                class: "w-5 h-5 accent-blue-500 bg-black",
                            }
                            label { r#for: "split", "Split" }
                        }

                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "direct",
                                class: "w-5 h-5 accent-blue-500",
                            }
                            label { r#for: "direct", "Direct" }
                        }
                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "direct",
                                class: "w-5 h-5 accent-blue-500",
                            }
                            label { r#for: "direct", "Direct" }
                        }
                    }
                }
            }

            div { class: "w-full max-w-md bg-black/10 backdrop-blur-md p-2 rounded-md shadow-2xl border border-gray-700 space-y-6 text-white/70",

                div {
                    label { "Tunnel Type" }
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "normal",
                                class: "w-5 h-5 text-red-600 bg-gray-100 border-gray-300 focus:ring-red-500 dark:focus:ring-red-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                                checked: true,
                            }
                            label { r#for: "normal", "Normal" }
                        }

                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "split",
                                class: "w-5 h-5 accent-blue-500 bg-black",
                            }
                            label { r#for: "split", "Split" }
                        }

                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "direct",
                                class: "w-5 h-5 accent-blue-500",
                            }
                            label { r#for: "direct", "Direct" }
                        }
                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "radio",
                                name: "payload_type",
                                id: "direct",
                                class: "w-5 h-5 accent-blue-500",
                            }
                            label { r#for: "direct", "Direct" }
                        }
                    }
                }
            }
            div { class: "w-full max-w-md bg-black/10 backdrop-blur-md p-2 rounded-md shadow-2xl border border-gray-700 space-y-6 text-white/70",
                p { "Options" }
                div { class: "flex flex-row gap-4",
                    input { r#type: "checkbox", class: "w-5 h-5 accent-blue-500" }
                    label { "Custom Payload" }
                }
            }
            Link { to: crate::views::app::Route::Home {},
                button { class: "w-full py-3 rounded-sm bg-white/20 text-sm", "SAVE" }
            }
        }
    }
}
