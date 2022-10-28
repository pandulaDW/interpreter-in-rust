pub mod objects;

#[derive(PartialEq, Eq)]
pub enum AllObjects {
    Integer(objects::Integer),
    Boolean(objects::Boolean),
    Null(objects::Null),
}

pub trait Object {
    fn inspect(&self) -> String;
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
