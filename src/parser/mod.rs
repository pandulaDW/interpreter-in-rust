#![allow(dead_code)]

mod helpers;
mod parse_expressions;
mod parse_statements;
pub mod program;

// Operator precedence
const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;
