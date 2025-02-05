use super::Teacher;

#[derive(Debug)]
pub struct Department {
    id: i64,

    full_name: String,
    short_name: String,

    teachers: Vec<Teacher>,
}
