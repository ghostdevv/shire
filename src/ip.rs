use color_eyre::eyre::Result;

pub async fn get_ip() -> Result<String> {
    let client = reqwest::Client::new();

    let ip = client
        .get("https://ip.willow.sh")
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_owned();

    Ok(ip)
}
