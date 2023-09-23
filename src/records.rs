use crate::cloudflare;
use color_eyre::eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Record {
    id: String,
    zone_name: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct RecordResponse {
    errors: Option<Vec<cloudflare::APIError>>,
    result: Option<Vec<Record>>,
}

pub async fn get_records(zone_id: &str, key: &str) -> Result<HashMap<String, String>> {
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A",
        zone_id
    );

    let client = reqwest::Client::new();

    let response = client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", key))
        .send()
        .await?
        .json::<RecordResponse>()
        .await?;

    cloudflare::assert_cf_errors(&response.errors, String::from("Failed to get records"))?;

    let map = response
        .result
        // It's safe to unwrap this here as we've already checked for errors
        // in the future I might try and figure out how to type this better
        .unwrap()
        .iter()
        .map(|record| {
            let parsed_name = record
                .name
                .trim_end_matches(&format!(".{}", record.zone_name))
                .to_owned();

            (parsed_name, record.id.to_owned())
        })
        .collect::<HashMap<_, _>>();

    Ok(map)
}

pub async fn set_ip(zone_id: &str, record_id: &str, ip: &str, key: &str) -> Result<()> {
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, record_id
    );

    let mut body = HashMap::new();
    body.insert("content", ip);

    let client = reqwest::Client::new();

    let response = client
        .patch(endpoint)
        .header("Authorization", format!("Bearer {}", key))
        .json(&body)
        .send()
        .await?
        .json::<cloudflare::BaseResponse>()
        .await?;

    cloudflare::assert_cf_errors(&response.errors, String::from("Failed to set ip"))?;

    Ok(())
}
