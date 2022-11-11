use std::{collections::HashMap, fmt::Display, hash::Hash};

use super::statements::BlockStatement;
use crate::lexer::{keywords, token};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AllExpressions {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    Assignment(AssignmentExpression),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(IndexExpression),
    RangeExpression(RangeExpression),
    HashLiteral(HashLiteral),
    NullLiteral,
}

impl Display for AllExpressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            AllExpressions::Identifier(v) => v.to_string(),
            AllExpressions::IntegerLiteral(v) => v.to_string(),
            AllExpressions::StringLiteral(v) => v.to_string(),
            AllExpressions::PrefixExpression(v) => v.to_string(),
            AllExpressions::InfixExpression(v) => v.to_string(),
            AllExpressions::Boolean(v) => v.to_string(),
            AllExpressions::IfExpression(v) => v.to_string(),
            AllExpressions::FunctionLiteral(v) => v.to_string(),
            AllExpressions::CallExpression(v) => v.to_string(),
            AllExpressions::ArrayLiteral(v) => v.to_string(),
            AllExpressions::NullLiteral => keywords::NULL.to_string(),
            AllExpressions::IndexExpression(v) => v.to_string(),
            AllExpressions::Assignment(v) => v.to_string(),
            AllExpressions::RangeExpression(v) => v.to_string(),
            AllExpressions::HashLiteral(v) => v.to_string(),
        };
        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    pub token: token::Token, // Ident token
    pub value: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IntegerLiteral {
    pub token: token::Token, // Int token
    pub value: i64,
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct StringLiteral {
    pub token: token::Token, // String token
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PrefixExpression {
    pub token: token::Token, // The prefix token, e.g. !
    pub operator: String,
    pub right: Option<Box<AllExpressions>>,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.right {
            return write!(f, "({}{})", self.operator, r);
        }
        write!(f, "")
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct InfixExpression {
    pub token: token::Token, // The infix token, e.g. !
    pub left: Option<Box<AllExpressions>>,
    pub operator: String,
    pub right: Option<Box<AllExpressions>>,
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AssignmentExpression {
    pub token: token::Token,
    pub ident: Identifier,
    pub value: Box<AllExpressions>,
}

impl Display for AssignmentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.ident, self.value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<AllExpressions>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|v| v.to_string()).collect();
        let mut out = String::new();
        out.push_str(format!("{}({}){}", self.token.literal, params.join(","), self.body).as_str());

        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CallExpression {
    pub token: token::Token,           // ( LPAREN
    pub function: Box<AllExpressions>, // Identifier or FunctionLiteral
    pub arguments: Vec<AllExpressions>,
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

// Cloning this structure is not a problem, as it is only the user defined portion of the (usually small) arrays
// that will get cloned across the program.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ArrayLiteral {
    pub token: token::Token,
    pub elements: Vec<AllExpressions>,
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .elements
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let out = format!("[{}]", elements);
        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IndexExpression {
    pub token: token::Token,
    pub left: Box<AllExpressions>,
    pub index: Box<AllExpressions>,
}

impl Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!("({}[{}])", self.left, self.index);
        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RangeExpression {
    pub token: token::Token,
    pub left: Box<AllExpressions>,
    pub left_index: Box<AllExpressions>,
    pub right_index: Box<AllExpressions>,
}

impl Display for RangeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!("({}[{}:{}])", self.left, self.left_index, self.right_index);
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct HashLiteral {
    pub token: token::Token,
    pub pairs: HashMap<AllExpressions, AllExpressions>,
}

impl PartialEq for HashLiteral {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

impl Eq for HashLiteral {}

impl Hash for HashLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}", self).hash(state)
    }
}

impl Display for HashLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs = self
            .pairs
            .iter()
            .map(|(key, item)| format!("{}:{}", key, item))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{{{}}}", pairs)
    }
}
