use crate::ast::expressions::Identifier;
use crate::ast::statements::{AllStatements, LetStatement, ReturnStatement, WhileStatement};
use crate::lexer::token::TokenType;

use super::parse_expressions::parse_block_statement;
use super::{program::Parser, Precedence};

impl Parser {
    /// The high level statement parser. Delegates the work to the relevant parsers
    pub fn parse_statement(&mut self) -> Option<AllStatements> {
        use TokenType::*;

        match self.current_token.token_type {
            Let => self.parse_let_statement(),
            Return => self.parse_return_statement(),
            While => self.parse_while_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Parses `Let` statements
    fn parse_let_statement(&mut self) -> Option<AllStatements> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let identifier = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = LetStatement {
            token,
            name: identifier,
            value,
        };

        Some(AllStatements::Let(stmt))
    }

    /// Parses `Return` statement
    fn parse_return_statement(&mut self) -> Option<AllStatements> {
        let token = self.current_token.clone();

        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = ReturnStatement {
            token,
            return_value,
        };

        Some(AllStatements::Return(stmt))
    }

    /// Parses `While` statements
    fn parse_while_statement(&mut self) -> Option<AllStatements> {
        let token_literal = self.current_token.clone();

        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }
        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = parse_block_statement(self);

        let stmt = WhileStatement {
            token: token_literal,
            condition,
            body,
        };

        Some(AllStatements::While(stmt))
    }
}
