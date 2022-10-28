use super::{statements::AllStatements, AllNodes, Node};
use std::fmt::Display;

/// Program node is going to be the root node of every AST that the parser produces
pub struct Program {
    pub statements: Vec<AllStatements>,
}

impl Program {
    /// Creates a new instance of the Program with parsed statements
    pub fn new() -> Self {
        Program { statements: vec![] }
    }

    /// Return the program as a variant of AllNodes to be evaluated by the evaluator
    pub fn make_node(self) -> AllNodes {
        AllNodes::Program(self)
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

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for stmt in self.statements.iter() {
            buf.push_str(format!("{}\n", stmt).as_str())
        }
        write!(f, "{}", buf)
    }
}
