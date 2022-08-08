#![allow(dead_code)]

use std::{any::Any, fmt::Display};

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

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str(format!("let {} = ", self.name).as_str());

        if let Some(v) = &self.value {
            out.push_str(v.to_string().as_str());
        }
        out.push(';');

        write!(f, "{}", out)
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

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str("return {}");

        if let Some(v) = &self.return_value {
            out.push_str(v.to_string().as_str());
        }
        out.push(';');

        write!(f, "{}", out)
    }
}

/// ExpressionStatement is not really a distinct statement; it’s a statement that
/// consists solely of one expression
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.expression {
            return write!(f, "{}", v);
        }
        write!(f, "")
    }
}
