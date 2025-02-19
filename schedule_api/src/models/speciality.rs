use super::Group;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Speciality {
    pub id: i64,

    pub full_name: String,
    pub short_name: String,

    pub groups: Option<Vec<Group>>,
}
