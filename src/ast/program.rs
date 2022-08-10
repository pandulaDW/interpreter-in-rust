use super::{Node, Statement};
use std::{any::Any, fmt::Display};

/// Program node is going to be the root node of every AST that the parser produces
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

impl Node for Program {
    /// Returns the token literal of the first statement
    fn token_literal(&self) -> String {
        match self.statements.get(0) {
            Some(s) => s.token_literal(),
            None => String::new(),
        }
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for stmt in self.statements.iter() {
            buf.push_str(format!("{}\n", stmt).as_str())
        }
        write!(f, "{}", buf)
    }
}
