#![allow(unused_imports)]

mod file_fetcher;
mod web_fetcher;

pub use file_fetcher::FileFetcher;
pub use web_fetcher::WebFetcher;

use crate::models::{AuditoryRoot, GroupRoot, PodrRoot};

pub trait Fetcher {
    async fn fetch_group(&self) -> Result<GroupRoot, Error>;

    // This one needs some manipulations on JSON because of the way the JSON is formatted by CIST
    async fn fetch_podr(&self) -> Result<PodrRoot, Error>;

    async fn fetch_auditories(&self) -> Result<AuditoryRoot, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to fetch data. {0} - {1}")]
    Fetch(String, String),

    #[error("Unexpected json format")]
    UnexpectedJsonFormat,
}
