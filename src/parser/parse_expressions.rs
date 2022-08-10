use super::{program::Parser, Precedence};
use crate::ast::expressions::{Identifier, IntegerLiteral, PrefixExpression};
use crate::ast::{statements::ExpressionStatement, Expression, Statement};
use crate::lexer::token::TokenType;

impl Parser {
    pub fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: None,
        };

        stmt.expression = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn Expression>> {
        // The key needed to be removed from the map to take ownership of the returned parser function.
        // This is to get around the borrow checker for passing a mutable reference of `self` to the parser function
        let prefix = match self.prefix_parse_fns.remove(&self.current_token.token_type) {
            Some(v) => v,
            None => {
                self.no_prefix_parse_fn_error(self.current_token.token_type.clone());
                return None;
            }
        };

        let left_expr = prefix(self);

        // The removed parser function will be added back here
        self.register_prefix(self.current_token.token_type.clone(), prefix);

        left_expr
    }
}

pub fn parse_identifier(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let ident = Identifier {
        token: p.current_token.clone(),
        value: p.current_token.literal.clone(),
    };

    Some(Box::new(ident))
}

pub fn parse_integer_literal(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let value = match p.current_token.literal.parse::<i64>() {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("could not parse as integer: {:?}", e);
            p.errors.push(msg);
            return None;
        }
    };

    let expr = IntegerLiteral {
        token: p.current_token.clone(),
        value,
    };

    Some(Box::new(expr))
}

pub fn parse_prefix_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut expr = PrefixExpression {
        token: p.current_token.clone(),
        operator: p.current_token.literal.clone(),
        right: None,
    };

    // advance the token to get the subject of the prefix
    p.next_token();

    expr.right = p.parse_expression(Precedence::Prefix);

    Some(Box::new(expr))
}
