use std::fmt::Display;

use super::{
    expressions::{self, AllExpressions},
    Node,
};
use crate::lexer::{keywords, token};

#[derive(Clone)]
pub enum AllStatements {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Node for AllStatements {
    fn token_literal(&self) -> String {
        match self {
            AllStatements::Let(v) => v.token_literal(),
            AllStatements::Return(v) => v.token_literal(),
            AllStatements::Expression(v) => v.token_literal(),
            AllStatements::Block(v) => v.token_literal(),
        }
    }
}

impl Display for AllStatements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            AllStatements::Let(v) => v.to_string(),
            AllStatements::Return(v) => v.to_string(),
            AllStatements::Expression(v) => v.to_string(),
            AllStatements::Block(v) => v.to_string(),
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct LetStatement {
    pub token: token::Token, // Let token
    pub name: expressions::Identifier,
    pub value: Box<AllExpressions>,
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

        out.push_str(self.value.to_string().as_str());
        out.push(';');

        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct ReturnStatement {
    pub token: token::Token, // Return token
    pub return_value: Box<AllExpressions>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        keywords::RETURN.to_string()
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str("return ");
        out.push_str(self.return_value.to_string().as_str());
        out.push(';');
        write!(f, "{}", out)
    }
}

/// ExpressionStatement is not really a distinct statement; it’s a statement that
/// consists solely of one expression
#[derive(Clone)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<AllExpressions>>,
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

#[derive(Clone)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<AllStatements>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = Vec::with_capacity(self.statements.len());
        for stmt in &self.statements {
            out.push(stmt.to_string());
        }
        write!(f, "{}", out.join("\n"))
    }
}
