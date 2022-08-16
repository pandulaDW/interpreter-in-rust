use std::{any::Any, fmt::Display};

use super::{Expression, Node};
use crate::lexer::token;

pub struct Identifier {
    pub token: token::Token, // Ident token
    pub value: String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct IntegerLiteral {
    pub token: token::Token, // Int token
    pub value: i64,
}

impl Expression for IntegerLiteral {}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct PrefixExpression {
    pub token: token::Token, // The prefix token, e.g. !
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for PrefixExpression {}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.right {
            return write!(f, "({}{})", self.operator, r);
        }
        write!(f, "")
    }
}

pub struct InfixExpression {
    pub token: token::Token, // The infix token, e.g. !
    pub left: Option<Box<dyn Expression>>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for InfixExpression {}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "(".to_string();

        if let Some(l) = &self.left {
            out.push_str(&l.to_string());
        }

        out.push_str(format!(" {} ", &self.operator).as_str());

        if let Some(r) = &self.right {
            out.push_str(format!("{})", &r).as_str());
        }

        write!(f, "{}", out)
    }
}

pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Expression for Boolean {}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}
