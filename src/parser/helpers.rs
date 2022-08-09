use super::program::{InfixParseFn, Parser, PrefixParseFn};
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
        self.peek_token.token_type == *token_type
    }

    /// Checks if the peek token is the supplied token type
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
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

    /// Insert the prefix parser function against the token type
    pub fn register_prefix(&mut self, token_type: TokenType, func: Box<PrefixParseFn>) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    /// Insert the infix parser function against the token type
    pub fn register_infix(&mut self, token_type: TokenType, func: Box<InfixParseFn>) {
        self.infix_parse_fns.insert(token_type, func);
    }
}
