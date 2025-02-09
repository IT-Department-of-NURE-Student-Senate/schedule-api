use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Teacher {
    id: i64,

    full_name: String,
    short_name: String,
}
