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
        // reading the first character of the input
        if self.read_position == 0 {
            self.read_char();
        }

        // skipping whitespace characters
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }

        // get the matching token
        let tok = match self.ch {
            '=' => new_token_from_ch(TokenType::Assign, self.ch),
            ';' => new_token_from_ch(TokenType::Semicolon, self.ch),
            '(' => new_token_from_ch(TokenType::Lparen, self.ch),
            ')' => new_token_from_ch(TokenType::Rparen, self.ch),
            ',' => new_token_from_ch(TokenType::Comma, self.ch),
            '+' => new_token_from_ch(TokenType::Plus, self.ch),
            '{' => new_token_from_ch(TokenType::Lbrace, self.ch),
            '}' => new_token_from_ch(TokenType::Rbrace, self.ch),
            NULL_CHAR => new_token_from_ch(TokenType::Eof, NULL_CHAR),
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let token_type = look_up_identifier(&identifier);
                    new_token_from_string(token_type, identifier)
                } else if self.ch.is_ascii_digit() {
                    let num_literal = self.read_number();
                    new_token_from_string(TokenType::Int, num_literal)
                } else {
                    new_token_from_ch(TokenType::Illegal, self.ch)
                }
            }
        };

        // read the next character
        self.read_char();

        tok
    }

    /// Sets the next character and advances the position in the input
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = NULL_CHAR;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Reads an identifier by iteratively calling the read_char method and returns the string literal
    fn read_identifier(&mut self) -> String {
        let current_position = self.position;
        while is_letter(self.peek_char()) {
            self.read_char();
        }
        self.input[current_position..self.read_position]
            .iter()
            .collect()
    }

    /// Reads a number by iteratively calling the read_char method and returns the number literal
    fn read_number(&mut self) -> String {
        let current_position = self.position;
        while self.peek_char().is_ascii_digit() {
            self.read_char();
        }
        self.input[current_position..self.read_position]
            .iter()
            .collect()
    }

    /// Returns the character corresponding to the read_position
    fn peek_char(&self) -> char {
        self.input[self.read_position]
    }
}

/// Checks if the given character is an ASCII alphabetic character or an _ character
fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
