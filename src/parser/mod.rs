mod helpers;
mod parse_expressions;
mod parse_statements;
pub mod program;

/// Operator precedences
#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    _EQUALS = 2,
    _LESSGREATER = 3,
    _SUM = 4,
    _PRODUCT = 5,
    Prefix = 6,
    _CALL = 7,
}
