use super::{program::Parser, Precedence};
use crate::ast::expressions::{self, Boolean, FunctionLiteral, Identifier, IfExpression};
use crate::ast::statements::BlockStatement;
use crate::ast::{statements::ExpressionStatement, Expression, Statement};
use crate::lexer::token::TokenType;

impl Parser {
    pub fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let trace_msg = self.tracer.trace("parseExpressionStatement");
        let mut stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: None,
        };

        stmt.expression = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        self.tracer.un_trace(trace_msg);
        Some(Box::new(stmt))
    }

    /// Uses pratt parse technique to parse a given expression.
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let trace_msg = self.tracer.trace("parseExpression");
        let prefix = match Parser::prefix_parse_function(&self.current_token.token_type) {
            Some(v) => v,
            None => {
                self.no_prefix_parse_fn_error(self.current_token.token_type.clone());
                return None;
            }
        };

        let mut left_expr = prefix(self);

        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = match Parser::infix_parse_function(&self.peek_token.token_type) {
                Some(v) => v,
                None => return left_expr,
            };

            self.next_token();

            left_expr = infix(self, left_expr);
        }

        self.tracer.un_trace(trace_msg);
        left_expr
    }
}

pub fn parse_identifier(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let ident = expressions::Identifier {
        token: p.current_token.clone(),
        value: p.current_token.literal.clone(),
    };

    Some(Box::new(ident))
}

pub fn parse_integer_literal(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let trace_msg = p.tracer.trace("parseIntegerLiteral");
    let value = match p.current_token.literal.parse::<i64>() {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("could not parse as integer: {:?}", e);
            p.errors.push(msg);
            return None;
        }
    };

    let expr = expressions::IntegerLiteral {
        token: p.current_token.clone(),
        value,
    };

    p.tracer.un_trace(trace_msg);
    Some(Box::new(expr))
}

pub fn parse_boolean_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let trace_msg = p.tracer.trace("parseBooleanLiteral");
    let bool_expr = Boolean {
        token: p.current_token.clone(),
        value: p.current_token_is(&TokenType::True),
    };
    p.tracer.un_trace(trace_msg);
    Some(Box::new(bool_expr))
}

pub fn parse_grouped_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let trace_msg = p.tracer.trace("parseGroupedExpression");
    p.next_token();

    let expr = p.parse_expression(Precedence::Lowest);
    if !p.expect_peek(TokenType::Rparen) {
        return None;
    }

    p.tracer.un_trace(trace_msg);
    expr
}

pub fn parse_prefix_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let trace_msg = p.tracer.trace("parsePrefixExpression");

    let mut expr = expressions::PrefixExpression {
        token: p.current_token.clone(),
        operator: p.current_token.literal.clone(),
        right: None,
    };

    // advance the token to get the subject of the prefix
    p.next_token();

    expr.right = p.parse_expression(Precedence::Prefix);

    p.tracer.un_trace(trace_msg);
    Some(Box::new(expr))
}

pub fn parse_infix_expression(
    p: &mut Parser,
    left: Option<Box<dyn Expression>>,
) -> Option<Box<dyn Expression>> {
    let tracer_msg = format!("parseInfixExpression {:?}", &p.current_token.literal);
    let trace_msg = p.tracer.trace(tracer_msg.as_str());

    let mut expression = expressions::InfixExpression {
        token: p.current_token.clone(),
        left,
        operator: p.current_token.literal.clone(),
        right: None,
    };

    let precedence = p.current_precedence();
    p.next_token();
    expression.right = p.parse_expression(precedence);

    p.tracer.un_trace(trace_msg);
    Some(Box::new(expression))
}

pub fn parse_if_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let token_literal = p.current_token.clone();

    if !p.expect_peek(TokenType::Lparen) {
        return None;
    }

    p.next_token();

    let condition = p.parse_expression(Precedence::Lowest)?;

    if !p.expect_peek(TokenType::Rparen) {
        return None;
    }

    if !p.expect_peek(TokenType::Lbrace) {
        return None;
    }

    let consequence = parse_block_statement(p);

    let mut alternative = None;
    if p.peek_token_is(&TokenType::Else) {
        p.next_token(); // consumes else
        if !p.expect_peek(TokenType::Lbrace) {
            return None;
        }
        alternative = Some(parse_block_statement(p));
    }

    Some(Box::new(IfExpression {
        token: token_literal,
        condition,
        consequence,
        alternative,
    }))
}

pub fn parse_function_literal(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = p.current_token.clone();

    if !p.expect_peek(TokenType::Lparen) {
        return None;
    }

    let parameters = parse_fn_literal_parameters(p)?;

    p.next_token();
    if !p.expect_peek(TokenType::Lbrace) {
        return None;
    }

    let body = parse_block_statement(p);

    Some(Box::new(FunctionLiteral {
        token,
        parameters,
        body,
    }))
}

fn parse_fn_literal_parameters(p: &mut Parser) -> Option<Vec<Identifier>> {
    let mut parameters = Vec::new();
    while !p.peek_token_is(&TokenType::Rparen) {
        if !p.expect_peek(TokenType::Ident) {
            return None;
        }
        let param = expressions::Identifier {
            token: p.current_token.clone(),
            value: p.current_token.literal.clone(),
        };
        parameters.push(param);

        if p.peek_token_is(&TokenType::Rparen) {
            break;
        }

        if !p.expect_peek(TokenType::Comma) {
            return None;
        }
    }
    Some(parameters)
}

fn parse_block_statement(p: &mut Parser) -> BlockStatement {
    let mut block = BlockStatement {
        token: p.current_token.clone(),
        statements: Vec::new(),
    };

    p.next_token();

    while !p.current_token_is(&TokenType::Rbrace) && !p.current_token_is(&TokenType::Eof) {
        let stmt = p.parse_statement();
        if let Some(v) = stmt {
            block.statements.push(v);
        };
        p.next_token();
    }

    block
}
