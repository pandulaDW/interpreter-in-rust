pub mod keywords;
pub mod token;
mod tokenizer;

/// Lexer is the structure that will tokenize a given input string
pub struct Lexer {
    /// input for the lexer (TODO: make it a readable interface)
    input: Vec<char>,

    /// current position in input (points to current char)
    position: usize,

    /// current reading position in input (after current char)
    read_position: usize,

    /// current character under examination
    ch: char,
}
