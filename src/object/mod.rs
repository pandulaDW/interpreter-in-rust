pub mod objects;

pub enum AllObjects {
    Integer(objects::Integer),
    _Boolean(objects::Boolean),
    _Null(objects::Null),
}

pub trait Object {
    fn inspect(&self) -> String;
}

impl Object for AllObjects {
    fn inspect(&self) -> String {
        match self {
            AllObjects::Integer(v) => v.inspect(),
            AllObjects::_Boolean(v) => v.inspect(),
            AllObjects::_Null(v) => v.inspect(),
        }
    }
}
