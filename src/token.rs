#![allow(dead_code)]

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

/// Represents the UNICODE null character
pub const NULL_CHAR: char = '\0';

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

/// Returns a new token based on the provided token type and the character
pub fn new_token_from_ch(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

/// Returns a new token based on the provided token type and the literal
pub fn new_token_from_string(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type,
        literal,
    }
}

/// Checks the given identifier and returns the correct TokeType.
pub fn look_up_identifier(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _ => TokenType::Ident,
    }
}

#[cfg(test)]
mod tests {
    use super::{new_token_from_ch, new_token_from_string, Token, TokenType::*, NULL_CHAR};
    use crate::lexer;

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
