pub mod objects;

pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(PartialEq, Eq)]
pub enum AllObjects {
    Integer(objects::Integer),
    Boolean(objects::Boolean),
    Null(objects::Null),
}

impl AllObjects {
    pub fn is_integer(&self) -> bool {
        if let AllObjects::Integer(_) = self {
            return true;
        }
        false
    }

    fn _is_boolean(&self) -> bool {
        if let AllObjects::Boolean(_) = self {
            return true;
        }
        false
    }

    fn _is_null(&self) -> bool {
        if let AllObjects::Null(_) = self {
            return true;
        }
        false
    }
}

impl Object for AllObjects {
    fn inspect(&self) -> String {
        match self {
            AllObjects::Integer(v) => v.inspect(),
            AllObjects::Boolean(v) => v.inspect(),
            AllObjects::Null(v) => v.inspect(),
        }
    }
}
