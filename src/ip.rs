use crate::utils;
use color_eyre::eyre::{eyre, Result};

pub async fn get_ip(resolver: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let response = client
        .get(resolver)
        .header("User-Agent", utils::get_ua_header())
        .send()
        .await?;

    // Check if the status code is 200
    if response.status() != reqwest::StatusCode::OK {
        return Err(eyre!(
            "The ip resolver failed with status code: {}",
            response.status()
        ));
    }

    let ip = response.text().await?.trim().to_owned();

    Ok(ip)
}
