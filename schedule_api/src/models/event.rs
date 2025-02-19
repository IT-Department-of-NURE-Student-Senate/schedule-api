use chrono::NaiveDateTime;
use serde::Deserialize;

use super::{Auditory, EventType, Group, Subject, Teacher};

#[derive(Debug, Deserialize)]
pub struct Event {
    id: i64,

    start_time: NaiveDateTime,
    end_time: NaiveDateTime,

    number_pair: u8,
    event_type: EventType,

    auditory: Auditory,
    subject: Subject,
    teachers: Vec<Teacher>,
    groups: Vec<Group>,
}
