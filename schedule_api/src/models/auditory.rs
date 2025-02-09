use super::AuditoryType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Auditory {
    id: i64,
    name: String,

    floor: u8,
    have_power: bool,
    auditory_types: Vec<AuditoryType>,
}
