use super::{Group, Speciality};

#[derive(Debug)]
pub struct Direction {
    id: i64,

    full_name: String,
    short_name: String,

    specialties: Vec<Speciality>,
    groups: Vec<Group>,
}
