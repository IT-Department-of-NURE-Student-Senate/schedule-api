use super::Group;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Speciality {
    id: i64,

    full_name: String,
    short_name: String,

    groups: Option<Vec<Group>>,
}
