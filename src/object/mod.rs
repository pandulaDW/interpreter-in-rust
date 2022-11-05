use std::fmt::{self, Display};

pub mod environment;
pub mod objects;

pub trait Object {
    fn inspect(&self) -> String;
}

/// This is useful when doing just type comparisons disregarding underlying value
#[derive(PartialEq, Eq)]
pub enum ObjectType {
    Integer,
    String,
    Boolean,
    Null,
    Error,
    Return,
    Function,
    BuiltInFunction,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            ObjectType::Integer => "INTEGER",
            ObjectType::String => "STRING",
            ObjectType::Boolean => "BOOLEAN",
            ObjectType::Null => "NULL",
            ObjectType::Error => "ERROR",
            ObjectType::Return => "RETURN",
            ObjectType::Function => "FUNCTION",
            ObjectType::BuiltInFunction => "BUILTIN_FUNCTION",
        };
        write!(f, "{}", out)
    }
}

/// A thin wrapper for all objects that implements `Object`.
///
/// This is the main structure that will be returned by all the evaluators.
#[derive(PartialEq, Eq, Clone)]
pub enum AllObjects {
    Integer(objects::Integer),
    StringObj(objects::StringObj),
    Boolean(objects::Boolean),
    Null(objects::Null),
    Error(objects::Error),
    ReturnValue(Box<AllObjects>),
    Function(objects::Function),
    BuiltinFunction(objects::BuiltinFunction),
}

impl Object for AllObjects {
    fn inspect(&self) -> String {
        match self {
            Self::Integer(v) => v.inspect(),
            Self::StringObj(v) => v.inspect(),
            Self::Boolean(v) => v.inspect(),
            Self::Null(v) => v.inspect(),
            Self::Error(v) => v.inspect(),
            Self::ReturnValue(v) => v.inspect(),
            Self::Function(v) => v.inspect(),
            Self::BuiltinFunction(v) => v.inspect(),
        }
    }
}

impl AllObjects {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Self::Integer(_) => ObjectType::Integer,
            Self::StringObj(_) => ObjectType::String,
            Self::Boolean(_) => ObjectType::Boolean,
            Self::Null(_) => ObjectType::Null,
            Self::Error(_) => ObjectType::Error,
            Self::ReturnValue(_) => ObjectType::Return,
            Self::Function(_) => ObjectType::Function,
            Self::BuiltinFunction(_) => ObjectType::Function,
        }
    }

    pub fn new_error(message: String) -> Self {
        Self::Error(objects::Error { message })
    }

    pub fn is_integer(&self) -> bool {
        self.object_type() == ObjectType::Integer
    }

    pub fn is_boolean(&self) -> bool {
        self.object_type() == ObjectType::Boolean
    }

    pub fn is_string(&self) -> bool {
        self.object_type() == ObjectType::String
    }

    pub fn is_error(&self) -> bool {
        self.object_type() == ObjectType::Error
    }
}
