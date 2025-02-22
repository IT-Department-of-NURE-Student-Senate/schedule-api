use super::{Group, Speciality};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Direction {
    pub id: i32,

    pub full_name: String,
    pub short_name: String,

    pub specialities: Option<Vec<Speciality>>,
    pub groups: Option<Vec<Group>>,
}
