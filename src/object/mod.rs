use self::objects::{Environment, Function, FunctionDefinition};
use crate::ast::expressions::FunctionLiteral;
use std::{
    fmt::{self, Display},
    rc::Rc,
};
use uuid::Uuid;

pub mod objects;

pub trait Object {
    fn inspect(&self) -> String;
}

/// This is useful when doing just type comparisons disregarding underlying value
#[derive(PartialEq, Eq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Error,
    Return,
    Function,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            ObjectType::Integer => "INTEGER",
            ObjectType::Boolean => "BOOLEAN",
            ObjectType::Null => "NULL",
            ObjectType::Error => "ERROR",
            ObjectType::Return => "RETURN",
            ObjectType::Function => "FUNCTION",
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
    Boolean(objects::Boolean),
    Null(objects::Null),
    Error(objects::Error),
    ReturnValue(Box<AllObjects>),
    Function(objects::Function),
}

impl Object for AllObjects {
    fn inspect(&self) -> String {
        match self {
            Self::Integer(v) => v.inspect(),
            Self::Boolean(v) => v.inspect(),
            Self::Null(v) => v.inspect(),
            Self::Error(v) => v.inspect(),
            Self::ReturnValue(v) => v.inspect(),
            Self::Function(v) => v.inspect(),
        }
    }
}

impl AllObjects {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Self::Integer(_) => ObjectType::Integer,
            Self::Boolean(_) => ObjectType::Boolean,
            Self::Null(_) => ObjectType::Null,
            Self::Error(_) => ObjectType::Error,
            Self::ReturnValue(_) => ObjectType::Return,
            Self::Function(_) => ObjectType::Function,
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

    pub fn is_error(&self) -> bool {
        self.object_type() == ObjectType::Error
    }

    /// Creates a new function object with a uniquely assigned name and a new environment
    pub fn new_function(node: FunctionLiteral) -> AllObjects {
        let name = format!("fn_{}", Uuid::new_v4());
        let env = Environment::new();

        let definition = FunctionDefinition {
            parameters: node.parameters,
            body: node.body,
            env,
        };

        Self::Function(Function {
            name,
            definition: Rc::new(definition),
        })
    }
}
