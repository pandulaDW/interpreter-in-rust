use super::Object;

#[derive(PartialEq, Eq)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(PartialEq, Eq)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(PartialEq, Eq)]
pub struct Null;

impl Object for Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(PartialEq, Eq)]
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn inspect(&self) -> String {
        format!("Error: {}", self.message)
    }
}
