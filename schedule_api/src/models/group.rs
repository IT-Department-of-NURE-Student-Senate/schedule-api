use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Group {
    pub id: i64,
    pub name: String,
}
