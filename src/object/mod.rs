pub mod objects;

pub trait Object {
    fn inspect(&self) -> String;
}

/// This is useful when doing just type comparisons disregarding underlying value
#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
    ERROR,
    RETURN,
}

#[derive(PartialEq, Eq)]
pub enum AllObjects {
    Integer(objects::Integer),
    Boolean(objects::Boolean),
    Null(objects::Null),
    Error(objects::Error),
    ReturnValue(Box<AllObjects>),
}

impl Object for AllObjects {
    fn inspect(&self) -> String {
        match self {
            Self::Integer(v) => v.inspect(),
            Self::Boolean(v) => v.inspect(),
            Self::Null(v) => v.inspect(),
            Self::Error(v) => v.inspect(),
            Self::ReturnValue(v) => v.inspect(),
        }
    }
}

impl AllObjects {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Self::Integer(_) => ObjectType::INTEGER,
            Self::Boolean(_) => ObjectType::BOOLEAN,
            Self::Null(_) => ObjectType::NULL,
            Self::Error(_) => ObjectType::ERROR,
            Self::ReturnValue(_) => ObjectType::RETURN,
        }
    }

    pub fn new_error(message: String) -> Self {
        Self::Error(objects::Error { message })
    }

    pub fn is_integer(&self) -> bool {
        self.object_type() == ObjectType::INTEGER
    }

    pub fn is_boolean(&self) -> bool {
        self.object_type() == ObjectType::BOOLEAN
    }

    pub fn is_error(&self) -> bool {
        self.object_type() == ObjectType::ERROR
    }
}
