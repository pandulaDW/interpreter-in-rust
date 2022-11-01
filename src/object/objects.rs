use super::{AllObjects, Object};
use crate::ast::{expressions::Identifier, statements::BlockStatement};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

/// Environment is what is used to keep track of values by associating them with an identifier.
///
/// It includes a store to collect variables of the main scope and an outer variable to keep track
/// of a function variables.
pub struct Environment {
    store: RefCell<HashMap<String, AllObjects>>,
    outer: Option<Rc<Environment>>,
}

impl Environment {
    /// Creates a new Environment
    pub fn new() -> Self {
        Environment {
            store: RefCell::new(HashMap::new()),
            outer: None,
        }
    }

    /// Enclose the environment with the given environment
    pub fn new_enclosed_environment(outer: Rc<Environment>) -> Environment {
        let mut new_env = Self::new();
        new_env.outer = Some(outer);
        new_env
    }

    /// Returns a clone of the `Object` corresponding to the `identifier` after recursively
    /// examining all the chained scopes.
    pub fn get(&self, name: &str) -> Option<AllObjects> {
        let binding = self.store.borrow();
        let obj = binding.get(name);
        let mut result = None;

        if obj.is_none() && self.outer.is_some() {
            if let Some(ref outer) = self.outer {
                result = outer.get(name);
            }
        } else if obj.is_some() {
            result = Some(obj.unwrap().clone());
        }

        result
    }

    /// Inserts a identifier-object pair into the store and return the passed object.
    ///
    /// The passed object will be cloned while inserting as they need to be persisted throughout the program.
    pub fn set(&self, name: String, value: AllObjects) -> AllObjects {
        self.store.borrow_mut().insert(name, value.clone());
        value
    }
}

/// Includes the name of the function and the definition of the function which wraps around in RC to be
/// clonable to fit the API of the other clonable objects.
///
/// This definition of object.Function has the Parameters and Body fields. But it also has Env,
/// a field that holds a pointer to an object.Environment, because functions in Monkey carry their
/// own environment with them. That allows for closures, which “close over” the environment they’re
/// defined in and can later access it.
#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub body: BlockStatement,
    pub parameters: Vec<Identifier>,
    pub env: Rc<Environment>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Function {}

impl Object for Function {
    fn inspect(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("fn({}){{\n{}\n}}", params, self.body)
    }
}
