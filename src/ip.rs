use reqwest::Error;

pub async fn get_ip() -> Result<String, Error> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://ip.willow.sh")
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
