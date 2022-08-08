#![allow(dead_code)]

use std::any::Any;

use super::{Expression, Node, Statement};
use crate::lexer::{keywords, token};

pub struct LetStatement {
    pub token: token::Token, // Let token
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        keywords::LET.to_string()
    }
}

pub struct Identifier {
    pub token: token::Token, // Ident token
    pub value: String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
