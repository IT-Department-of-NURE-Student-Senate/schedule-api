use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Teacher {
    pub id: i64,

    pub full_name: String,
    pub short_name: String,
}
