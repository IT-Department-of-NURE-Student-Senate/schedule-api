use serde::Deserialize;

use super::University;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub university: University,
}
