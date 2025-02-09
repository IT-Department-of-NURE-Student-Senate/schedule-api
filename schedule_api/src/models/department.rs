use super::Teacher;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Department {
    id: i64,

    full_name: String,
    short_name: String,

    teachers: Option<Vec<Teacher>>,
}
