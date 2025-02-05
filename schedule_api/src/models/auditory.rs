use super::AuditoryType;

#[derive(Debug)]
pub struct Auditory {
    id: i64,
    name: String,

    floor: u8,
    have_power: bool,
    auditory_types: Vec<AuditoryType>,
}
