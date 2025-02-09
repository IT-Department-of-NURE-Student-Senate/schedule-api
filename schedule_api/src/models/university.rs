use super::{Building, Faculty};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct University {
    pub full_name: String,
    pub short_name: String,

    pub faculties: Option<Vec<Faculty>>,
    pub buildings: Option<Vec<Building>>,
}
