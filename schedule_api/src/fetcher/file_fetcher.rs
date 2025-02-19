use log::{error, info};
use regex::Regex;

use crate::models::AuditoryRoot;
use crate::models::GroupRoot;
use crate::models::PodrRoot;

use super::Error;
use super::Fetcher;

#[derive(Debug)]
pub struct FileFetcher {
    podr_re: Regex,
}

impl FileFetcher {
    pub fn new() -> Self {
        let re_str = r#"(teachers":\[[^\]]*\]\})\n,\n(\{\n"id":\d+,"short_name":".+?","full_name":".+?","departments":\[)"#;
        let podr_re = Regex::new(re_str).unwrap(); // Should never fail, so unwrap is fine

        Self { podr_re }
    }
}

impl Fetcher for FileFetcher {
    async fn fetch_group(&self) -> Result<GroupRoot, Error> {
        let json_string = std::fs::read_to_string("../json/GROUP.json")?;

        let root: GroupRoot = serde_json::from_str(&json_string)?;

        Ok(root)
    }

    async fn fetch_podr(&self) -> Result<PodrRoot, Error> {
        let json_string = std::fs::read_to_string("../json/PODR.json")?;

        let fixed_json = self.podr_re.replace(&json_string, "$1]}, $2");

        let root: PodrRoot = serde_json::from_str(&fixed_json)?;

        Ok(root)
    }

    async fn fetch_auditories(&self) -> Result<AuditoryRoot, Error> {
        let json_string = std::fs::read_to_string("../json/AUDITORIES.json")?;

        let root: AuditoryRoot = serde_json::from_str(&json_string)?;

        Ok(root)
    }
}
