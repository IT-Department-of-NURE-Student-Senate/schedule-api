use serde::Deserialize;

use super::{AuditoryUniversity, GroupUniversity, PodrUniversity};

#[derive(Debug, Deserialize)]
pub struct PodrRoot {
    pub university: PodrUniversity,
}

#[derive(Debug, Deserialize)]
pub struct GroupRoot {
    pub university: GroupUniversity,
}

#[derive(Debug, Deserialize)]
pub struct AuditoryRoot {
    pub university: AuditoryUniversity,
}
