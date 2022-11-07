use crate::lexer::token::TokenType;

mod helpers;
mod parse_expressions;
mod parse_statements;
mod program;
mod tracing;

pub use program::Parser;
pub static mut TRACING_ENABLED: bool = false;

/// Operator precedences
#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
    Index = 8,
}

impl Precedence {
    fn corresponding_precedence(token_type: &TokenType) -> Precedence {
        use Precedence::*;
        use TokenType::*;

        match token_type {
            Eq | NotEq => Equals,
            Lt | Gt => LessGreater,
            Plus | Minus => Sum,
            Slash | Asterisk => Product,
            Lparen => Call,
            Lbracket => Index,
            _ => Lowest,
        }
    }
}
