use super::{AllObjects, Object};
use std::collections::HashMap;

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
