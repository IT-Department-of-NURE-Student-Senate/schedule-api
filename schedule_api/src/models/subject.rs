use super::{EventType, Teacher};

#[derive(Debug)]
pub struct Subject {
    id: i64,

    short_name: String,
    full_name: String,

    hours_by_teacher: Vec<(Teacher, u8, EventType)>,
}
