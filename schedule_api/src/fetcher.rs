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
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
}
