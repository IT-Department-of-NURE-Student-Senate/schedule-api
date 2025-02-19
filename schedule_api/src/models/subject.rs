use serde::Deserialize;

use super::{EventType, Teacher};

type TeacherHours = (i64, u8, i64);

#[derive(Debug, Deserialize)]
pub struct Subject {
    id: i64,

    short_name: String,
    full_name: String,

    hours_by_teacher: Vec<TeacherHours>,
}
