use super::{Group, Speciality};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Direction {
    id: i64,

    full_name: String,
    short_name: String,

    specialties: Option<Vec<Speciality>>,
    groups: Option<Vec<Group>>,
}
