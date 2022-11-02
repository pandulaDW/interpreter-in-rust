use super::AllObjects;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

    /// Enclose a new environment with the given environment
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
