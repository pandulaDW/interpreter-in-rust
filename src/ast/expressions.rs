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
