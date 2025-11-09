use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IpResponse {
    ip: String,
}

pub async fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ipify.org?format=json")
        .await?
        .json::<IpResponse>()
        .await?;

    Ok(response.ip)
}
