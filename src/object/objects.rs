use super::{environment::Environment, AllObjects, Object};
use crate::ast::{expressions::Identifier, statements::BlockStatement};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct StringObj {
    pub value: Rc<String>,
}

impl Object for StringObj {
    fn inspect(&self) -> String {
        self.value.replace("\\n", "\n").replace("\\t", "\t")
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Null;

impl Object for Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn inspect(&self) -> String {
        format!("Error: {}", self.message)
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
pub struct FunctionObj {
    pub name: String,
    pub body: BlockStatement,
    pub parameters: Vec<Identifier>,
    pub env: Rc<Environment>,
}

impl PartialEq for FunctionObj {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for FunctionObj {}

impl Hash for FunctionObj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Object for FunctionObj {
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct BuiltinFunctionObj {
    pub fn_name: String,
    pub parameters: ParamsType,
    pub func: fn(Rc<Environment>) -> AllObjects,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ParamsType {
    Fixed(Vec<String>),
    Variadic,
}

impl Object for BuiltinFunctionObj {
    fn inspect(&self) -> String {
        self.fn_name.to_string()
    }
}

#[derive(Clone)]
pub struct ArrayObj {
    pub elements: Rc<RefCell<Vec<AllObjects>>>,
}

impl PartialEq for ArrayObj {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl Eq for ArrayObj {}

impl Hash for ArrayObj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inspect().hash(state)
    }
}

impl Object for ArrayObj {
    fn inspect(&self) -> String {
        format!(
            "[{}]",
            self.elements
                .borrow()
                .iter()
                .map(|v| v.inspect())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Clone)]
pub struct HashMapObj {
    pub map: Rc<RefCell<HashMap<AllObjects, AllObjects>>>,
}

impl PartialEq for HashMapObj {
    fn eq(&self, other: &Self) -> bool {
        self.inspect() == other.inspect()
    }
}

impl Eq for HashMapObj {}

impl Hash for HashMapObj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inspect().hash(state)
    }
}

impl Object for HashMapObj {
    fn inspect(&self) -> String {
        let binding = self.map.borrow();
        let out = binding
            .iter()
            .map(|(k, v)| format!("{}:{}", k.inspect(), v.inspect()))
            .collect::<Vec<String>>()
            .join(", ");

        format!("{{ {} }}", out)
    }
}
