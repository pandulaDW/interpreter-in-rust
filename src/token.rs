#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
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

    // Keywords
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

/// Returns a new token based on the provided token type and the character
pub fn new_token_from_ch(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

/// Returns a new token based on the provided token type and the literal
pub fn new_token_from_string(token_type: TokenType, literal: &str) -> Token {
    Token {
        token_type,
        literal: literal.to_string(),
    }
}

/// Checks the given identifier and returns the correct TokeType.
pub fn look_up_identifier(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Ident,
    }
}

#[cfg(test)]
mod tests {
    use super::{look_up_identifier, TokenType};

    #[test]
    fn test_look_up_look_up_identifier() {
        assert_eq!(TokenType::Function, look_up_identifier("fn"));
        assert_eq!(TokenType::Let, look_up_identifier("let"));
        assert_eq!(TokenType::Ident, look_up_identifier("my name is khan"));
    }
}
