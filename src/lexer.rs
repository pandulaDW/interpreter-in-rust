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
    /// Returns a new Lexer after reading the first character
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: NULL_CHAR,
        };

        l.read_char();

        return l;
    }

    /// Returns the current token and reads the next token which can be returned on the
    /// next call
    pub fn next_token(&mut self) -> Token {
        let tok;

        match self.ch {
            '=' => tok = new_token(TokenType::ASSIGN, self.ch),
            ';' => tok = new_token(TokenType::SEMICOLON, self.ch),
            '(' => tok = new_token(TokenType::LPAREN, self.ch),
            ')' => tok = new_token(TokenType::RPAREN, self.ch),
            ',' => tok = new_token(TokenType::COMMA, self.ch),
            '+' => tok = new_token(TokenType::PLUS, self.ch),
            '{' => tok = new_token(TokenType::LBRACE, self.ch),
            '}' => tok = new_token(TokenType::RBRACE, self.ch),
            _ => tok = new_token(TokenType::EOF, NULL_CHAR),
        }

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
