use super::Auditory;

#[derive(Debug)]
pub struct Building {
    id: i64,

    full_name: String,
    short_name: String,

    auditories: Vec<Auditory>,
}
