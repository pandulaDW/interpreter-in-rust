use std::fmt::Display;

use super::expressions::{self, AllExpressions};
use crate::lexer::token;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AllStatements {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
    While(WhileStatement),
}

impl Display for AllStatements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            AllStatements::Let(v) => v.to_string(),
            AllStatements::Return(v) => v.to_string(),
            AllStatements::Expression(v) => v.to_string(),
            AllStatements::Block(v) => v.to_string(),
            AllStatements::While(v) => v.to_string(),
        };

        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct LetStatement {
    pub token: token::Token, // Let token
    pub name: expressions::Identifier,
    pub value: Box<AllExpressions>,
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ReturnStatement {
    pub token: token::Token, // Return token
    pub return_value: Box<AllExpressions>,
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
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<AllExpressions>>,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.expression {
            return write!(f, "{}", v);
        }
        write!(f, "")
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<AllStatements>,
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct WhileStatement {
    pub token: token::Token,
    pub condition: Box<AllExpressions>,
    pub body: BlockStatement,
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!("while ({}) {{\n {} }}", self.condition, self.body);
        write!(f, "{}", out)
    }
}
