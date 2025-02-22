use super::Group;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Speciality {
    pub id: i32,

    pub full_name: String,
    pub short_name: String,

    pub groups: Vec<Group>,
}
