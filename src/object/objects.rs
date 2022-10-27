use super::{Object, ObjectType};

struct Integer {
    value: i64,
}

impl Object for Integer {
    fn object_type() -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

struct Boolean {
    value: bool,
}

impl Object for Boolean {
    fn object_type() -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

struct Null {}

impl Object for Null {
    fn object_type() -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}
