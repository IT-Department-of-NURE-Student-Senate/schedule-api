use encoding_rs::WINDOWS_1251;
use log::{error, info};
use regex::Regex;

use crate::models::{AuditoryRoot, GroupRoot, PodrRoot};

use super::{Error, Fetcher};

const BASE_URL: &str = "https://cist.nure.ua/ias/app/tt";

#[derive(Debug)]
pub struct WebFetcher {
    client: reqwest::Client,
    podr_re: regex::Regex,
}

impl WebFetcher {
    pub async fn new() -> Result<Self, Error> {
        let client = reqwest::Client::builder().build()?;

        let re_str = r#"(teachers":\[[^\]]*\]\})\n,\n(\{\n"id":\d+,"short_name":".+?","full_name":".+?","departments":\[)"#;
        let podr_re = Regex::new(re_str).unwrap(); // Should never fail, so unwrap is fine

        Ok(WebFetcher { client, podr_re })
    }

    async fn fetch(&self, url: &str) -> Result<String, Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            let reason = status.canonical_reason().unwrap_or("No description");

            error!("Failed to fetch data from {}. {} - {}", url, status, reason);
            return Err(Error::Fetch(status.to_string(), reason.to_string()));
        }

        let response_bytes = response.bytes().await?;

        let (decoded, _, _) = encoding_rs::WINDOWS_1251.decode(response_bytes.as_ref());
        info!("Decoded {} bytes", decoded.len());

        Ok(decoded.into_owned())
    }
}

impl Fetcher for WebFetcher {
    async fn fetch_group(&self) -> Result<GroupRoot, Error> {
        let fetched_data = self.fetch(&format!("{BASE_URL}/P_API_GROUP_JSON")).await?;

        let root: GroupRoot = serde_json::from_str(&fetched_data)?;

        Ok(root)
    }

    async fn fetch_podr(&self) -> Result<PodrRoot, Error> {
        let fetched_data = self.fetch(&format!("{BASE_URL}/P_API_PODR_JSON")).await?;

        let fixed_data = self.podr_re.replace(&fetched_data, "$1]}, $2").to_string();

        let root: PodrRoot = serde_json::from_str(&fixed_data)?;

        Ok(root)
    }

    async fn fetch_auditories(&self) -> Result<AuditoryRoot, Error> {
        let fetched_data = self
            .fetch(&format!("{BASE_URL}/P_API_AUDITORIES_JSON"))
            .await?;

        let root: AuditoryRoot = serde_json::from_str(&fetched_data)?;

        Ok(root)
    }
}
