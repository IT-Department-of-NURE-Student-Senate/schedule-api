use super::Teacher;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Department {
    pub id: i64,

    pub full_name: String,
    pub short_name: String,

    pub teachers: Option<Vec<Teacher>>,
}
