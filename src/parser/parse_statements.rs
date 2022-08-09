use crate::ast::expressions::Identifier;
use crate::ast::statements::{LetStatement, ReturnStatement};
use crate::ast::Statement;
use crate::lexer::keywords;
use crate::lexer::token::{new_token, TokenType};

use super::program::Parser;

impl Parser {
    /// The high level statement parser. Delegates the work to the relevant parsers
    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        use TokenType::*;

        match self.current_token.token_type {
            Let => self.parse_let_statement(),
            Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
    }

    /// Parses `Let` statements
    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
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

        // TODO: skipping the expressions until encountering a semicolon
        while !self.current_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = LetStatement {
            token: new_token(TokenType::Let, keywords::LET),
            name: identifier,
            value: None,
        };

        Some(Box::new(stmt))
    }

    /// Parses `Return` statement
    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ReturnStatement {
            token: new_token(TokenType::Return, keywords::RETURN),
            return_value: None,
        };

        // TODO: skipping the expressions until encountering a semicolon
        while !self.current_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }
}
