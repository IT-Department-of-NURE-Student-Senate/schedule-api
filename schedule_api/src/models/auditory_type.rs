use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuditoryType {
    id: i64,
    name: String,
}
