use crate::token::*;

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

impl Lexer {
    /// Returns a new Lexer for the given input
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: NULL_CHAR,
        }
    }

    /// Returns the current token and reads the next token which will be returned on the
    /// next call
    pub fn next_token(&mut self) -> Token {
        // read the first character of the input
        if self.read_position == 0 {
            self.read_char();
        }

        let tok = match self.ch {
            '=' => new_token(TokenType::Assign, self.ch),
            ';' => new_token(TokenType::Semicolon, self.ch),
            '(' => new_token(TokenType::Lparen, self.ch),
            ')' => new_token(TokenType::Rparen, self.ch),
            ',' => new_token(TokenType::Comma, self.ch),
            '+' => new_token(TokenType::Plus, self.ch),
            '{' => new_token(TokenType::Lbrace, self.ch),
            '}' => new_token(TokenType::Rbrace, self.ch),
            _ => new_token(TokenType::Eof, NULL_CHAR),
        };

        // read the next character
        self.read_char();

        tok
    }

    /// Sets the next character and advances the position in the input
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = NULL_CHAR;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}
