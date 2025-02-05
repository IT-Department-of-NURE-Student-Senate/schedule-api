use super::{Department, Direction};

#[derive(Debug)]
pub struct Faculty {
    id: i64,

    full_name: String,
    short_name: String,

    departments: Vec<Department>,
    directions: Vec<Direction>,
}
