use super::{Building, FacultyWithDepartments, FacultyWithDirections};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PodrUniversity {
    pub full_name: String,
    pub short_name: String,

    pub faculties: Vec<FacultyWithDepartments>,
}

#[derive(Debug, Deserialize)]
pub struct GroupUniversity {
    pub full_name: String,
    pub short_name: String,

    pub faculties: Vec<FacultyWithDirections>,
}

#[derive(Debug, Deserialize)]
pub struct AuditoryUniversity {
    pub full_name: String,
    pub short_name: String,

    pub buildings: Vec<Building>,
}
