use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Group {
    id: i64,
    name: String,
}
