#[derive(Debug, Clone)]
pub enum Object {
    Unit,
    Int(i32),
    String(String),
    Boolean(bool),
    Char(char),
    Array(Vec<Object>),
    Range(Box<Object>, Box<Object>),
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Int(int) => int.to_string(),
            Object::String(string) => string.to_owned(),
            Object::Boolean(bool) => bool.to_string(),
            Object::Char(char) => char.to_string(),
            _ => unreachable!(),
        }
    }
}
