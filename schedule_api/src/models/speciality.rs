use super::Group;

#[derive(Debug)]
pub struct Speciality {
    id: i64,

    full_name: String,
    short_name: String,

    groups: Vec<Group>,
}
