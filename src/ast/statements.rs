#![allow(dead_code)]

use super::{Expression, Node, Statement};
use crate::lexer::token;

pub struct LetStatement {
    pub token: token::Token, // Let token
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

pub struct Identifier {
    pub token: token::Token, // Ident token
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
