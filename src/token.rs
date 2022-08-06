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
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

/// Generates a new token based on the provided token type and the character
pub fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenType::*, NULL_CHAR};
    use crate::lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let test_cases: Vec<Token> = vec![
            Token {
                token_type: ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: EOF,
                literal: NULL_CHAR.to_string(),
            },
        ];

        let mut l = lexer::Lexer::new(input);

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
}
