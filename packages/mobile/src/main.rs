use tracing::info;
use ui::App;

#[tokio::main]
async fn main() {
    if cfg!(target_os = "android") {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Info), // Set the desired log level
        );
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .without_time()
            .init();
    }

    info!("Starting RustyVPN...");

    let mut ssh_config = ssh::config::SshConfig::default();
    ssh_config.host = "57.158.81.120".to_string();
    ssh_config.port = 22;
    ssh_config.username = "monir".to_string();
    ssh_config.password = "12345".to_string();
    ssh_config.is_proxy_enabled = false;

    ssh::config::SSH_CONFIG.get_or_init(|| ssh_config);

    dioxus::launch(App);
}
