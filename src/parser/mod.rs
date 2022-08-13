use crate::lexer::token::TokenType;

mod helpers;
mod parse_expressions;
mod parse_statements;
pub mod program;
mod tracing;

pub static mut TRACING_ENABLED: bool = false;

/// Operator precedences
#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    _Call = 7,
}

impl Precedence {
    fn corresponding_token_type(token_type: &TokenType) -> Precedence {
        use Precedence::*;
        use TokenType::*;

        match token_type {
            Eq | NotEq => Equals,
            Lt | Gt => LessGreater,
            Plus | Minus => Sum,
            Slash | Asterisk => Product,
            _ => Lowest,
        }
    }
}
