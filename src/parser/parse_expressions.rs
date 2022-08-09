use super::program::Parser;
use super::LOWEST;
use crate::ast::expressions::Identifier;
use crate::ast::{statements::ExpressionStatement, Expression, Statement};
use crate::lexer::token::TokenType;

impl Parser {
    pub fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: None,
        };

        stmt.expression = self.parse_expression(LOWEST);

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, _precedence: u8) -> Option<Box<dyn Expression>> {
        let prefix = match self.prefix_parse_fns.get(&self.current_token.token_type) {
            Some(v) => v,
            None => return None,
        };

        let left_expr = prefix(self);

        Some(left_expr)
    }
}

pub fn parse_identifier(p: &Parser) -> Box<dyn Expression> {
    let ident = Identifier {
        token: p.current_token.clone(),
        value: p.current_token.literal.clone(),
    };

    Box::new(ident)
}
