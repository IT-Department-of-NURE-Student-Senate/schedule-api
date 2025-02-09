use super::Auditory;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Building {
    id: i64,

    full_name: String,
    short_name: String,

    auditories: Vec<Auditory>,
}
