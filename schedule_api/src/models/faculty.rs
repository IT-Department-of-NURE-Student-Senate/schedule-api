use super::{Department, Direction};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Faculty {
    id: i64,

    full_name: String,
    short_name: String,

    departments: Option<Vec<Department>>,
    directions: Option<Vec<Direction>>,
}
