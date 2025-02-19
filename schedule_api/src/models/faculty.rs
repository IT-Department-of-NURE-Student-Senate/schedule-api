use super::{Department, Direction};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FacultyWithDepartments {
    pub id: i64,

    pub full_name: String,
    pub short_name: String,

    pub departments: Option<Vec<Department>>,
}

#[derive(Debug, Deserialize)]
pub struct FacultyWithDirections {
    pub id: i64,

    pub full_name: String,
    pub short_name: String,

    pub directions: Option<Vec<Direction>>,
}
