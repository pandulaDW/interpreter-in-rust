#![allow(dead_code)]

use super::{Node, Statement};

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
}
