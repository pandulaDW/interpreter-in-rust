mod helpers;
mod parse_expressions;
mod parse_statements;
pub mod program;

#[allow(dead_code)]
// Operator precedences
mod precedence {
    pub const LOWEST: u8 = 1;
    pub const EQUALS: u8 = 2;
    pub const LESSGREATER: u8 = 3;
    pub const SUM: u8 = 4;
    pub const PRODUCT: u8 = 5;
    pub const PREFIX: u8 = 6;
    pub const CALL: u8 = 7;
}
