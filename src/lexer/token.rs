use super::keywords;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

/// Represents the UNICODE null character
pub const NULL_CHAR: char = '\0';

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,
    String,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Colon,

    // Keywords
    Function,
    Let,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
}

/// A helper function to return an EOF token for initializing the parser
pub fn eof_token() -> Token {
    new_token(TokenType::Eof, NULL_CHAR)
}

/// Returns a new token based on the provided token type and the literal
pub fn new_token<T: ToString>(token_type: TokenType, literal: T) -> Token {
    Token {
        token_type,
        literal: literal.to_string(),
    }
}

/// Checks the given identifier and returns the correct `TokenType`.
pub fn look_up_identifier(ident: &str) -> TokenType {
    use keywords::*;

    match ident {
        FN => TokenType::Function,
        LET => TokenType::Let,
        IF => TokenType::If,
        ELSE => TokenType::Else,
        RETURN => TokenType::Return,
        WHILE => TokenType::While,
        TRUE => TokenType::True,
        FALSE => TokenType::False,
        NULL => TokenType::Null,
        _ => TokenType::Ident,
    }
}

#[cfg(test)]
mod tests {
    use super::{look_up_identifier, TokenType};

    #[test]
    fn test_look_up_identifier() {
        assert_eq!(TokenType::Function, look_up_identifier("fn"));
        assert_eq!(TokenType::Let, look_up_identifier("let"));
        assert_eq!(TokenType::Ident, look_up_identifier("my name is khan"));
    }
}
