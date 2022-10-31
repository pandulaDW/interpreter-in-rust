use super::{AllObjects, Object};
use crate::ast::{expressions::Identifier, statements::BlockStatement};
use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Null;

impl Object for Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn inspect(&self) -> String {
        format!("Error: {}", self.message)
    }
}

/// Environment is what is used to keep track of values by associating them with an identifier
pub struct Environment {
    store: HashMap<String, AllObjects>,
}

impl Environment {
    /// Creates a new Environment
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    /// Returns a reference to the `Object` corresponding to the `identifier`.
    pub fn get(&self, name: &str) -> Option<&AllObjects> {
        self.store.get(name)
    }

    /// Inserts a identifier-object pair into the store and return the passed object.
    ///
    /// The passed object will be cloned while inserting as they need to be persisted throughout the program.
    pub fn set(&mut self, name: String, value: AllObjects) -> AllObjects {
        self.store.insert(name, value.clone());
        value
    }
}

/// Includes the name of the function and the definition of the function which wraps around in RC to be
/// clonable to fit the API of the other clonable objects.
pub struct Function {
    pub name: String,
    pub definition: Rc<FunctionDefinition>,
}

/// This definition of object.Function has the Parameters and Body fields. But it also has Env,
/// a field that holds a pointer to an object.Environment, because functions in Monkey carry their
/// own environment with them. That allows for closures, which “close over” the environment they’re
/// defined in and can later access it.
pub struct FunctionDefinition {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Function {}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            definition: self.definition.clone(),
        }
    }
}

impl Object for Function {
    fn inspect(&self) -> String {
        let params = self
            .definition
            .parameters
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("fn({}){{\n{}\n}}", params, self.definition.body)
    }
}
