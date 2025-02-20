use super::{Department, Direction};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FacultyWithDepartments {
    pub id: i32,

    pub full_name: String,
    pub short_name: String,

    pub departments: Vec<Department>,
}

#[derive(Debug, Deserialize)]
pub struct FacultyWithDirections {
    pub id: i32,

    pub full_name: String,
    pub short_name: String,

    pub directions: Vec<Direction>,
}
