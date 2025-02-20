use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Group {
    pub id: i32,
    pub name: String,
}
