use super::{Building, Faculty};

#[derive(Debug)]
pub struct University {
    full_name: String,
    short_name: String,

    faculties: Vec<Faculty>,
    buildings: Vec<Building>,
}
