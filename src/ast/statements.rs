#![allow(dead_code)]

use std::any::Any;

use super::{expressions, Expression, Node, Statement};
use crate::lexer::{keywords, token};

pub struct LetStatement {
    pub token: token::Token, // Let token
    pub name: expressions::Identifier,
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

pub struct ReturnStatement {
    pub token: token::Token, // Return token
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        keywords::RETURN.to_string()
    }
}
