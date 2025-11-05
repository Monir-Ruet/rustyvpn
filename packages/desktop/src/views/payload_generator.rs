use dioxus::prelude::*;

const NEXT_ICON: Asset = asset!("/assets/icons/next.png");

#[component]
pub fn PayloadGenerator() -> Element {
    rsx! {
        div { class: "min-h-screen bg-black/92 justify-center p-5",
            div { class: "flex flex-col mx-auto gap-4 text-white/70",
                div {
                    Link {
                        to: crate::views::app::Route::TunnelType {
                        },
                        button { class: "w-full text-white bg-white/15",
                            div { class: "flex items-center justify-between p-4",
                                p { class: "text-xs text-white/60",
                                    "HTTP Proxy -> SSH (Custom Payload)"
                                }
                                button {
                                    img { class: "w-4 h-4", src: NEXT_ICON }
                                }
                            }
                        }
                    }
                }
                div {
                    label { "Payload" }
                    div { class: "flex flex-row gap-10 items-center",
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
                    }
                    div { class: "flex flex-row gap-4 items-center mt-4 justify-between",
                        div { class: "flex flex-row gap-4 items-center",
                            input {
                                r#type: "checkbox",
                                id: "rotate",
                                class: "w-5 h-5 accent-blue-500",
                            }
                            label { r#for: "rotate", "Rotate" }
                        }
                        div {
                            select { class: "w-40 text-white p-2",
                                option { "H" }
                                option { "V" }
                                option { "T" }
                            }
                        }
                    }
                }

                div {
                    input {
                        class: "border-1 border-[#ccc] p-4 w-full text-white font-mono rounded-md ",
                        placeholder: "URL/Host",
                    }
                }
                div { class: "flex flex-row justify-between",
                    div { class: "flex flex-col gap-4",
                        label { class: "text-left", "Request Method" }
                        select { class: "w-40 text-white p-2",
                            option { "GET" }
                            option { "POST" }
                            option { "PUT" }
                            option { "DELETE" }
                            option { "PATCH" }
                        }
                    }
                    div { class: "flex flex-col gap-4",
                        label { "Injection Method" }
                        select { class: "w-40 text-white p-2",
                            option { "URL Injection" }
                            option { "Header Injection" }
                            option { "Both" }
                        }
                    }
                }
                label { "Query Methods" }
                div { class: "grid grid-cols-2 gap-4",
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "front_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "front_query", "Front Query" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Back Query" }
                    }
                }

                label { "Extra Headers" }
                div { class: "grid grid-cols-2 gap-4",
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Online Host" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Forward Host" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Reverse Proxy" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Keep Alive" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "User Agent" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Referer" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "back_query",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "back_query", "Websocket" }
                    }
                }

                label { "Raw" }
                div { class: "grid grid-cols-2 gap-4",
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "raw",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "raw", "Raw" }
                    }
                    div { class: "flex flex-row gap-4",
                        input {
                            r#type: "checkbox",
                            id: "raw",
                            class: "w-5 h-5 accent-blue-500",
                        }
                        label { r#for: "raw", "Dual Connect" }
                    }
                }

                Link { to: crate::views::app::Route::Home {},
                    button { class: "w-full py-3 rounded-sm bg-white/20 text-sm", "GENERATE PAYLOAD" }
                }
            }
        }
    }
}
