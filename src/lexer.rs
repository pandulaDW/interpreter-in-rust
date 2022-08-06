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

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token::{TokenType::*, *};

    #[test]
    fn test_next_token_for_characters() {
        let input = "=+(){},;";
        let mut l = lexer::Lexer::new(input);

        let test_cases: Vec<Token> = vec![
            new_token_from_ch(Assign, '='),
            new_token_from_ch(Plus, '+'),
            new_token_from_ch(Lparen, '('),
            new_token_from_ch(Rparen, ')'),
            new_token_from_ch(Lbrace, '{'),
            new_token_from_ch(Rbrace, '}'),
            new_token_from_ch(Comma, ','),
            new_token_from_ch(Semicolon, ';'),
            new_token_from_ch(Eof, NULL_CHAR),
        ];

        for (i, tt) in test_cases.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(
                tt.token_type, tok.token_type,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, tt.token_type, tok.token_type
            );

            assert_eq!(
                tt.literal, tok.literal,
                "tests[{}] - token type wrong. expected={}, got={}",
                i, tt.literal, tok.literal
            );
        }
    }

    #[test]
    fn test_next_token_source_code() {
        let input = "let five = 5;
                    let ten = 10;
                    let add = fn(x, y) {
                        x + y;
                    };
                    let result = add(five, ten);";
        let mut l = lexer::Lexer::new(input);

        let test_cases = vec![
            new_token_from_string(Let, "let".to_string()),
            new_token_from_string(Ident, "five".to_string()),
            new_token_from_ch(Assign, '='),
            new_token_from_string(Int, "5".to_string()),
            new_token_from_ch(Semicolon, ';'),
            new_token_from_string(Let, "let".to_string()),
            new_token_from_string(Ident, "ten".to_string()),
            new_token_from_ch(Assign, '='),
            new_token_from_string(Int, "10".to_string()),
            new_token_from_ch(Semicolon, ';'),
            new_token_from_string(Let, "let".to_string()),
            new_token_from_string(Ident, "add".to_string()),
            new_token_from_ch(Assign, '='),
            new_token_from_string(Function, "fn".to_string()),
            new_token_from_ch(Lparen, '('),
            new_token_from_string(Ident, "x".to_string()),
            new_token_from_ch(Comma, ','),
            new_token_from_string(Ident, "y".to_string()),
            new_token_from_ch(Rparen, ')'),
            new_token_from_ch(Lbrace, '{'),
            new_token_from_string(Ident, "x".to_string()),
            new_token_from_ch(Plus, '+'),
            new_token_from_string(Ident, "y".to_string()),
            new_token_from_ch(Semicolon, ';'),
            new_token_from_ch(Rbrace, '}'),
            new_token_from_ch(Semicolon, ';'),
            new_token_from_string(Let, "let".to_string()),
            new_token_from_string(Ident, "result".to_string()),
            new_token_from_ch(Assign, '='),
            new_token_from_string(Ident, "add".to_string()),
            new_token_from_ch(Lparen, '('),
            new_token_from_string(Ident, "five".to_string()),
            new_token_from_ch(Comma, ','),
            new_token_from_string(Ident, "ten".to_string()),
            new_token_from_ch(Rparen, ')'),
            new_token_from_ch(Semicolon, ';'),
            new_token_from_ch(Eof, NULL_CHAR),
        ];

        for (i, tt) in test_cases.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(
                tt.literal, tok.literal,
                "tests[{}] - token type wrong. expected={}, got={}",
                i, tt.literal, tok.literal
            );

            assert_eq!(
                tt.token_type, tok.token_type,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, tt.token_type, tok.token_type
            );
        }
    }
}
