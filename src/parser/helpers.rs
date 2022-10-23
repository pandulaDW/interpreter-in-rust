use super::{program::Parser, Precedence};
use crate::lexer::token::TokenType;

use std::mem;

impl Parser {
    /// Sets the peek token to the current token and advance to a new token.
    pub fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.l.next_token();
    }

    /// Checks if the current token is the supplied token type
    pub fn current_token_is(&self, token_type: &TokenType) -> bool {
        self.current_token.token_type == *token_type
    }

    /// Checks if the peek token is the supplied token type
    pub fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == *token_type
    }

    /// Return true If the peek token is the supplied token type and advance the tokenizer
    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            return true;
        }
        self.peek_error(token_type);
        false
    }

    /// Add an error to errors when the type of peekToken doesn't match the expectation
    pub fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token
        );
        self.errors.push(msg);
    }

    pub fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", token_type);
        self.errors.push(msg);
    }

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::corresponding_precedence(&self.peek_token.token_type)
    }

    pub fn current_precedence(&self) -> Precedence {
        Precedence::corresponding_precedence(&self.current_token.token_type)
    }
}
