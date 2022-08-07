#![allow(dead_code)]

use super::{Expression, Node, Statement};
use crate::lexer::token;

/// Program node is going to be the root node of every AST that the parser produces
struct Program<T: Statement> {
    statements: Vec<T>,
}

impl<T: Statement> Node for Program<T> {
    /// Returns the token literal of the first statement
    fn token_literal(&self) -> String {
        match self.statements.get(0) {
            Some(s) => s.token_literal(),
            None => String::new(),
        }
    }
}

struct LetStatement<T: Expression> {
    token: token::Token, // Let token
    name: Identifier,
    value: T,
}

impl<T: Expression> Statement for LetStatement<T> {
    fn statement_node() {}
}

impl<T: Expression> Node for LetStatement<T> {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

struct Identifier {
    token: token::Token, // Ident token
    value: String,
}

impl Expression for Identifier {
    fn expression_node() {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
