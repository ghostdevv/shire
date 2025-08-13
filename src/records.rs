use crate::{cloudflare, utils};
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Zone {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ZoneResponse {
    errors: Option<Vec<cloudflare::APIError>>,
    result: Option<Zone>,
}

async fn get_zone(zone_id: &str, key: &str) -> Result<Zone> {
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/zones/{}",
        zone_id
    );

    let client = reqwest::Client::new();

    let response = client
        .get(endpoint)
        .header("User-Agent", utils::get_ua_header())
        .header("Authorization", format!("Bearer {}", key))
        .send()
        .await?
        .json::<ZoneResponse>()
        .await?;

    cloudflare::assert_cf_errors(&response.errors, String::from("Failed to get zone"))?;

    Ok(response.result.unwrap())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub content: String,
    pub comment: Option<String>
}

#[derive(Deserialize, Debug)]
struct RecordResponse {
    errors: Option<Vec<cloudflare::APIError>>,
    result: Option<Vec<Record>>,
}

pub async fn get_records(zone_id: &str, key: &str) -> Result<HashMap<String, Record>> {
    let zone = get_zone(zone_id, key).await?;

    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A",
        zone_id
    );

    let client = reqwest::Client::new();

    let records_response = client
        .get(endpoint)
        .header("User-Agent", utils::get_ua_header())
        .header("Authorization", format!("Bearer {}", key))
        .send()
        .await?
        .json::<RecordResponse>()
        .await?;

    cloudflare::assert_cf_errors(&records_response.errors, String::from("Failed to get records"))?;

    let map = records_response
        .result
        // It's safe to unwrap this here as we've already checked for errors
        // in the future I might try and figure out how to type this better
        .unwrap()
        .iter_mut()
        .map(|record| {
            record.name = record
                .name
                .trim_end_matches(&format!(".{}", zone.name))
                .to_owned();

            (record.name.clone(), record.to_owned())
        })
        .collect::<HashMap<_, _>>();

    Ok(map)
}

#[derive(Serialize, Debug)]
pub struct NewRecord {
    pub name: String,
    #[serde(rename(serialize = "type"))]
    pub record_type: String,
    pub content: String,
    pub proxied: bool,
    pub comment: String,
}

pub struct UpdateRecordsBodyBuilder {
    ip: String,
    patches: Vec<Record>,
    posts: Vec<NewRecord>,
}

impl UpdateRecordsBodyBuilder {
    pub fn new(ip: String) -> Self {
        Self {
            ip,
            patches: vec![],
            posts: vec![],
        }
    }

    pub fn create(&mut self, name: String) {
        self.posts.push(NewRecord {
            name,
            record_type: String::from("A"),
            content: self.ip.to_owned(),
            proxied: false,
            comment: String::from("created by shire"),
        });
    }

    pub fn update(&mut self, record_id: String, name: String, comment: Option<String>) {
        self.patches.push(Record {
            id: record_id,
            name,
            content: self.ip.to_owned(),
            comment: comment.or(Some(String::from("created by shire"))),
        });
    }
}

pub async fn update_records(zone_id: &str, key: &str, body: &UpdateRecordsBodyBuilder) -> Result<()> {
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/batch",
        zone_id
    );

    let client = reqwest::Client::new();

    let response = client
        .post(endpoint)
        .header("User-Agent", utils::get_ua_header())
        .header("Authorization", format!("Bearer {}", key))
        .json(&json!({ "patches": body.patches, "posts": body.posts }))
        .send()
        .await?
        .json::<cloudflare::BaseResponse>()
        .await?;

    cloudflare::assert_cf_errors(&response.errors, String::from("Failed to update records"))?;

    Ok(())
}
