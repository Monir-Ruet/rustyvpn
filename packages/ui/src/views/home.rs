use crate::route::Route;
use anyhow::Result;
use dioxus::prelude::*;
use jni::{
    objects::{JObject, JValue},
    JavaVM,
};
use ndk_context::android_context;
use reqwest;
use serde::Deserialize;
use socks::SocksBuilder;
use ssh::SshBuilder;
use tokio_util::sync::CancellationToken;
use tracing::error;

const PAYLOAD_GENERATE_ICON: Asset = asset!("/assets/icons/code.png");

#[derive(Clone, Copy, PartialEq)]
enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Deserialize)]
struct IpResponse {
    ip: String,
}

async fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ipify.org?format=json")
        .await?
        .json::<IpResponse>()
        .await?;

    Ok(response.ip)
}

pub fn call_java_max(a: i32, b: i32) -> String {
    unsafe {
        let ctx = android_context();
        let vm = JavaVM::from_raw(ctx.vm().cast()).unwrap();
        let mut env = vm.attach_current_thread().unwrap();
        let result = env
            .call_static_method(
                "java/lang/Math",
                "max",
                "(II)I",
                &[JValue::Int(a), JValue::Int(b)],
            )
            .unwrap()
            .i()
            .unwrap();

        result.to_string()
    }
}

pub fn request_permission(permission: &str) -> Result<()> {
    unsafe {
        let ctx = android_context();
        let vm = JavaVM::from_raw(ctx.vm().cast())?;
        let mut env = vm.attach_current_thread()?;
        let activity = JObject::from_raw(ctx.context() as *mut _);

        if activity.is_null() {
            anyhow::bail!("Err");
        }

        let permission_jstring = env.new_string(permission)?;
        let permission_jobject = JObject::from(permission_jstring);
        let string_class = env.find_class("java/lang/String")?;
        let permissions_array = env.new_object_array(1, string_class, JObject::null())?;
        env.set_object_array_element(&permissions_array, 0, permission_jobject)?;
        env.call_method(
            activity,
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[JValue::Object(&permissions_array), JValue::Int(1)],
        )?;

        info!("requestPermissions called");

        Ok(())
    }
}

pub fn check_if_permission_granted(permission: &str) -> Result<bool> {
    unsafe {
        let ctx = android_context();
        let vm = JavaVM::from_raw(ctx.vm().cast())?;
        let mut env = vm.attach_current_thread()?;
        let activity = JObject::from_raw(ctx.context() as *mut _);

        if activity.is_null() {
            anyhow::bail!("Err");
        }

        let permission_jstring = env.new_string(permission)?;
        let permission_jobject = JObject::from(permission_jstring);

        let result = env.call_method(
            activity,
            "checkSelfPermission",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&permission_jobject)],
        )?;

        let granted = result.i()? == 0;

        Ok(granted)
    }
}

#[component]
pub fn Home() -> Element {
    let mut cancellation_token = use_signal(|| CancellationToken::new());
    let mut connection_status = use_signal(|| ConnectionStatus::Disconnected);
    let connection_status_clone = connection_status.read().clone();
    let mut ip = use_signal(|| "".to_string());
    let ip_clone = ip.read().clone();
    // let x = use_signal(|| call_java_max(10, 50));
    // let x_clone = x.read().clone();

    #[inline]
    async fn connect_ssh_and_proxy(cancellation_token: CancellationToken) -> Result<()> {
        let mut ssh = SshBuilder::new().build();
        ssh.connect().await?;
        ssh.authenticate().await?;
        tokio::spawn(async move {
            if let Err(e) = SocksBuilder::new()
                .with_cancellation_token(cancellation_token)
                .build()
                .run()
                .await
            {
                error!("Dynamic port forwarding error: {:?}", e);
            }
        });
        Ok(())
    }

    let handle_connection = move |_| {
        if connection_status_clone != ConnectionStatus::Disconnected {
            cancellation_token.read().clone().cancel();
            connection_status.set(ConnectionStatus::Disconnected);
        } else {
            cancellation_token.set(CancellationToken::new());
            spawn(async move {
                loop {
                    connection_status.set(ConnectionStatus::Connecting);
                    if let Err(_) = connect_ssh_and_proxy(cancellation_token.read().clone()).await {
                        connection_status.set(ConnectionStatus::Disconnected);
                        continue;
                    }
                    if let Ok(retrieved_ip) = get_public_ip().await {
                        ip.set(retrieved_ip);
                    }
                    connection_status.set(ConnectionStatus::Connected);
                    break;
                }
            });
        }
    };

    rsx! {
        div { class: "min-h-screen bg-black/90 text-gray-100 flex flex-col items-center justify-center p-6",
            // div { "{x_clone}" }
            button {
                onclick: |_| {
                    if (!check_if_permission_granted("android.permission.READ_CONTACTS").unwrap()) {
                        info!("Permission not granted");
                    } else {
                        request_permission("android.permission.READ_CONTACTS").unwrap();
                    }
                },
                "Get Contacts"
            }
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

                    button {
                        class: "w-full py-3 rounded-md bg-smoke/20 active:scale-95 transition-all duration-200 font-semibold text-md shadow-md shadow-blue-900/99 border-1 border-blue-500",
                        disabled: connection_status_clone == ConnectionStatus::Connecting,
                        onclick: handle_connection,
                        match connection_status_clone {
                            ConnectionStatus::Disconnected => "Connect",
                            ConnectionStatus::Connecting => "Connecting...",
                            ConnectionStatus::Connected => "Disconnect",
                        }
                    }
                }

                div { class: "border-t border-gray-800 pt-4 space-y-2 text-sm text-gray-400",
                    div { class: "flex justify-between items-center",
                        span { "Public IP:" }
                        span { class: "text-gray-200 font-mono",
                            match connection_status_clone {
                                ConnectionStatus::Connected => ip_clone.as_str(),
                                _ => "Not connected",
                            }
                        }
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
