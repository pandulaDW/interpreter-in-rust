use super::token::*;
use super::Lexer;

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
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    new_token(TokenType::Eq, "==")
                } else {
                    new_token(TokenType::Assign, self.ch)
                }
            }
            ';' => new_token(TokenType::Semicolon, self.ch),
            '(' => new_token(TokenType::Lparen, self.ch),
            ')' => new_token(TokenType::Rparen, self.ch),
            ',' => new_token(TokenType::Comma, self.ch),
            '+' => new_token(TokenType::Plus, self.ch),
            '-' => new_token(TokenType::Minus, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    new_token(TokenType::NotEq, "!=")
                } else {
                    new_token(TokenType::Bang, self.ch)
                }
            }
            '*' => new_token(TokenType::Asterisk, self.ch),
            '/' => new_token(TokenType::Slash, self.ch),
            '<' => new_token(TokenType::Lt, self.ch),
            '>' => new_token(TokenType::Gt, self.ch),
            '{' => new_token(TokenType::Lbrace, self.ch),
            '}' => new_token(TokenType::Rbrace, self.ch),
            '"' => new_token(TokenType::String, self.read_string()),
            '[' => new_token(TokenType::Lbracket, self.ch),
            ']' => new_token(TokenType::Rbracket, self.ch),
            ':' => new_token(TokenType::Colon, self.ch),
            NULL_CHAR => new_token(TokenType::Eof, NULL_CHAR),
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let token_type = look_up_identifier(&identifier);
                    new_token(token_type, &identifier)
                } else if self.ch.is_ascii_digit() {
                    let num_literal = self.read_number();
                    new_token(TokenType::Int, num_literal)
                } else {
                    new_token(TokenType::Illegal, self.ch)
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
        while is_letter_or_digit(self.peek_char()) {
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

    /// Reads a string by iteratively calling the read_char method and returns the string literal
    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == NULL_CHAR {
                break;
            }
        }
        self.input[position..self.position].iter().collect()
    }

    /// Returns the character corresponding to the read_position
    fn peek_char(&self) -> char {
        self.input
            .get(self.read_position)
            .unwrap_or(&NULL_CHAR)
            .to_owned()
    }
}

/// Checks if the given character is an ASCII alphabetic character or an _ character
fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

/// Checks if the given character is an ASCII alphabetic character, _ or a digit
fn is_letter_or_digit(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::lexer::token::{TokenType::*, *};

    #[test]
    fn test_next_token_for_characters() {
        let input = "=+(){},;";
        let mut l = Lexer::new(input);

        let test_cases: Vec<Token> = vec![
            new_token(Assign, '='),
            new_token(Plus, '+'),
            new_token(Lparen, '('),
            new_token(Rparen, ')'),
            new_token(Lbrace, '{'),
            new_token(Rbrace, '}'),
            new_token(Comma, ','),
            new_token(Semicolon, ';'),
            new_token(Eof, NULL_CHAR),
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
        let input = r#"let five = 5;
                    let ten = 10;
                    let add = fn(x, y) {
                        x + y;
                    };
                    let result = add(five, ten);
                    
                    !-/*5;
                5 < 10 > 5;
                if (5 < 10) {
                    return true;
                } else {
                    return false;
                }
                10 == 10; 
                10 != 9;
                "foobar"
                "foo bar"
                [1, 2];
                let x = null;
                {"foo": "bar"}
                "#;
        let mut l = Lexer::new(input);

        let test_cases = vec![
            new_token(Let, "let"),
            new_token(Ident, "five"),
            new_token(Assign, '='),
            new_token(Int, "5"),
            new_token(Semicolon, ';'),
            new_token(Let, "let"),
            new_token(Ident, "ten"),
            new_token(Assign, '='),
            new_token(Int, "10"),
            new_token(Semicolon, ';'),
            new_token(Let, "let"),
            new_token(Ident, "add"),
            new_token(Assign, '='),
            new_token(Function, "fn"),
            new_token(Lparen, '('),
            new_token(Ident, "x"),
            new_token(Comma, ','),
            new_token(Ident, "y"),
            new_token(Rparen, ')'),
            new_token(Lbrace, '{'),
            new_token(Ident, "x"),
            new_token(Plus, '+'),
            new_token(Ident, "y"),
            new_token(Semicolon, ';'),
            new_token(Rbrace, '}'),
            new_token(Semicolon, ';'),
            new_token(Let, "let"),
            new_token(Ident, "result"),
            new_token(Assign, '='),
            new_token(Ident, "add"),
            new_token(Lparen, '('),
            new_token(Ident, "five"),
            new_token(Comma, ','),
            new_token(Ident, "ten"),
            new_token(Rparen, ')'),
            new_token(Semicolon, ';'),
            new_token(Bang, '!'),
            new_token(Minus, '-'),
            new_token(Slash, '/'),
            new_token(Asterisk, '*'),
            new_token(Int, '5'),
            new_token(Semicolon, ';'),
            new_token(Int, '5'),
            new_token(Lt, '<'),
            new_token(Int, "10"),
            new_token(Gt, '>'),
            new_token(Int, '5'),
            new_token(Semicolon, ';'),
            new_token(If, "if"),
            new_token(Lparen, '('),
            new_token(Int, '5'),
            new_token(Lt, '<'),
            new_token(Int, "10"),
            new_token(Rparen, ')'),
            new_token(Lbrace, '{'),
            new_token(Return, "return"),
            new_token(True, "true"),
            new_token(Semicolon, ';'),
            new_token(Rbrace, '}'),
            new_token(Else, "else"),
            new_token(Lbrace, '{'),
            new_token(Return, "return"),
            new_token(False, "false"),
            new_token(Semicolon, ';'),
            new_token(Rbrace, '}'),
            new_token(Int, "10"),
            new_token(Eq, "=="),
            new_token(Int, "10"),
            new_token(Semicolon, ';'),
            new_token(Int, "10"),
            new_token(NotEq, "!="),
            new_token(Int, "9"),
            new_token(Semicolon, ';'),
            new_token(String, "foobar"),
            new_token(String, "foo bar"),
            new_token(Lbracket, "["),
            new_token(Int, "1"),
            new_token(Comma, ","),
            new_token(Int, "2"),
            new_token(Rbracket, "]"),
            new_token(Semicolon, ';'),
            new_token(Let, "let"),
            new_token(Ident, "x"),
            new_token(Assign, '='),
            new_token(Null, "null"),
            new_token(Semicolon, ';'),
            new_token(Lbrace, '{'),
            new_token(String, "foo"),
            new_token(Colon, ':'),
            new_token(String, "bar"),
            new_token(Rbrace, '}'),
            new_token(Eof, NULL_CHAR),
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
