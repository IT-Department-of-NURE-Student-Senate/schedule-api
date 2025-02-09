use log::{error, info};

use crate::models::{Root, University};

#[derive(Debug)]
pub struct Fetcher {
    base_url: String,
    client: reqwest::Client,
}

impl Fetcher {
    pub fn new(base_url: &str) -> Result<Self, Error> {
        let client = reqwest::Client::builder().build()?;

        Ok(Fetcher {
            base_url: base_url.to_string(),
            client,
        })
    }

    pub async fn fetch_groups(&self) -> Result<University, Error> {
        let response = self
            .client
            .get(format!("{}/{}", self.base_url, "P_API_GROUP_JSON"))
            .send()
            .await?;

        let bytes = response.bytes().await?;
        let (decoded, _, _) = encoding_rs::WINDOWS_1251.decode(&bytes); // Convert from CP1251 to UTF-8

        info!("Decoded {} bytes", bytes.len());

        let root: Root = serde_json::from_str(&decoded).map_err(|e| {
            error!("JSON parsing error: {}", e);
            e
        })?;

        Ok(root.university)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
