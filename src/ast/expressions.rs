use std::fmt::Display;

use super::{statements::BlockStatement, Node};
use crate::lexer::token;

#[derive(Clone)]
pub enum AllExpressions {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}

impl Display for AllExpressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            AllExpressions::Identifier(v) => v.to_string(),
            AllExpressions::IntegerLiteral(v) => v.to_string(),
            AllExpressions::PrefixExpression(v) => v.to_string(),
            AllExpressions::InfixExpression(v) => v.to_string(),
            AllExpressions::Boolean(v) => v.to_string(),
            AllExpressions::IfExpression(v) => v.to_string(),
            AllExpressions::FunctionLiteral(v) => v.to_string(),
            AllExpressions::CallExpression(v) => v.to_string(),
        };
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct PrefixExpression {
    pub token: token::Token, // The prefix token, e.g. !
    pub operator: String,
    pub right: Option<Box<AllExpressions>>,
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

#[derive(Clone)]
pub struct InfixExpression {
    pub token: token::Token, // The infix token, e.g. !
    pub left: Option<Box<AllExpressions>>,
    pub operator: String,
    pub right: Option<Box<AllExpressions>>,
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<AllExpressions>,
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct CallExpression {
    pub token: token::Token,           // ( LPAREN
    pub function: Box<AllExpressions>, // Identifier or FunctionLiteral
    pub arguments: Vec<AllExpressions>,
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
