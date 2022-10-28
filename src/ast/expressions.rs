use std::fmt::Display;

use super::{statements::BlockStatement, Node};
use crate::lexer::token;

pub enum AllExpression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}

impl Display for AllExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            AllExpression::Identifier(v) => v.to_string(),
            AllExpression::IntegerLiteral(v) => v.to_string(),
            AllExpression::PrefixExpression(v) => v.to_string(),
            AllExpression::InfixExpression(v) => v.to_string(),
            AllExpression::Boolean(v) => v.to_string(),
            AllExpression::IfExpression(v) => v.to_string(),
            AllExpression::FunctionLiteral(v) => v.to_string(),
            AllExpression::CallExpression(v) => v.to_string(),
        };
        write!(f, "{}", out)
    }
}

pub struct Identifier {
    pub token: token::Token, // Ident token
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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
    pub right: Option<Box<AllExpression>>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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
    pub left: Option<Box<AllExpression>>,
    pub operator: String,
    pub right: Option<Box<AllExpression>>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<AllExpression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut consequence = String::new();
        for line in self.consequence.to_string().lines() {
            consequence.push_str(format!("  {};\n", line).as_str());
        }

        let mut out = format!("if {} {{ \n{}}}", self.condition, consequence);

        match &self.alternative {
            Some(v) => out.push_str(format!("else {}", v).as_str()),
            None => {}
        };

        write!(f, "{}", out)
    }
}

pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|v| v.to_string()).collect();
        let mut out = String::new();
        out.push_str(
            format!(
                "{}({}){}",
                self.token_literal(),
                params.join(","),
                self.body
            )
            .as_str(),
        );

        write!(f, "{}", out)
    }
}

pub struct CallExpression {
    pub token: token::Token, // ( LPAREN
    pub function: Box<AllExpression>,
    pub arguments: Vec<AllExpression>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!(
            "{}({})",
            self.function,
            self.arguments
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        write!(f, "{}", out)
    }
}
