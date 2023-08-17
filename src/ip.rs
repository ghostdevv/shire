use color_eyre::eyre::Result;

pub async fn get_ip(resolver: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let ip = client
        .get(resolver)
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_owned();

    Ok(ip)
}
