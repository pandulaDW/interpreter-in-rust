use super::{program::Parser, Precedence};
use crate::ast::expressions::{
    self, AllExpressions, ArrayLiteral, Boolean, CallExpression, FunctionLiteral, Identifier,
    IfExpression, IndexExpression, StringLiteral,
};
use crate::ast::statements::ExpressionStatement;
use crate::ast::statements::{AllStatements, BlockStatement};
use crate::lexer::token::TokenType;

/// A type alias for the optional boxed expression type that is commonly used in parser functions
type BoxedExpression = Option<Box<AllExpressions>>;

impl Parser {
    pub fn parse_expression_statement(&mut self) -> Option<AllStatements> {
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
        Some(AllStatements::Expression(stmt))
    }

    /// Uses pratt parse technique to parse a given expression.
    pub fn parse_expression(&mut self, precedence: Precedence) -> BoxedExpression {
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

pub fn parse_identifier(p: &mut Parser) -> BoxedExpression {
    let ident = expressions::Identifier {
        token: p.current_token.clone(),
        value: p.current_token.literal.clone(),
    };

    Some(Box::new(AllExpressions::Identifier(ident)))
}

pub fn parse_integer_literal(p: &mut Parser) -> BoxedExpression {
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
    Some(Box::new(AllExpressions::IntegerLiteral(expr)))
}

pub fn parse_string_literal(p: &mut Parser) -> BoxedExpression {
    let trace_msg = p.tracer.trace("parseStringLiteral");
    let str_literal = StringLiteral {
        token: p.current_token.clone(),
    };
    p.tracer.un_trace(trace_msg);
    Some(Box::new(AllExpressions::StringLiteral(str_literal)))
}

pub fn parse_null_literal(p: &mut Parser) -> BoxedExpression {
    let trace_msg = p.tracer.trace("parseNullLiteral");
    p.tracer.un_trace(trace_msg);
    Some(Box::new(AllExpressions::NullLiteral))
}

pub fn parse_boolean_expression(p: &mut Parser) -> BoxedExpression {
    let trace_msg = p.tracer.trace("parseBooleanLiteral");
    let bool_expr = Boolean {
        token: p.current_token.clone(),
        value: p.current_token_is(&TokenType::True),
    };
    p.tracer.un_trace(trace_msg);
    Some(Box::new(AllExpressions::Boolean(bool_expr)))
}

pub fn parse_grouped_expression(p: &mut Parser) -> BoxedExpression {
    let trace_msg = p.tracer.trace("parseGroupedExpression");
    p.next_token();

    let expr = p.parse_expression(Precedence::Lowest);
    if !p.expect_peek(TokenType::Rparen) {
        return None;
    }

    p.tracer.un_trace(trace_msg);
    expr
}

pub fn parse_prefix_expression(p: &mut Parser) -> BoxedExpression {
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
    Some(Box::new(AllExpressions::PrefixExpression(expr)))
}

pub fn parse_infix_expression(p: &mut Parser, left: BoxedExpression) -> BoxedExpression {
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
    Some(Box::new(AllExpressions::InfixExpression(expression)))
}

pub fn parse_if_expression(p: &mut Parser) -> BoxedExpression {
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

    let if_expr = IfExpression {
        token: token_literal,
        condition,
        consequence,
        alternative,
    };

    Some(Box::new(AllExpressions::IfExpression(if_expr)))
}

pub fn parse_function_literal(p: &mut Parser) -> BoxedExpression {
    let token = p.current_token.clone();

    if !p.expect_peek(TokenType::Lparen) {
        return None;
    }

    let parameters = parse_fn_literal_parameters(p)?;

    p.next_token(); // consumes )
    if !p.expect_peek(TokenType::Lbrace) {
        return None;
    }

    let body = parse_block_statement(p);

    let fn_literal = FunctionLiteral {
        token,
        parameters,
        body,
    };

    Some(Box::new(AllExpressions::FunctionLiteral(fn_literal)))
}

pub fn parse_call_expression(p: &mut Parser, left: BoxedExpression) -> BoxedExpression {
    let token = p.current_token.clone(); // (
    let function = left?;

    let arguments = parse_comma_sep_arguments(p, &TokenType::Rparen)?;
    p.next_token(); // consumes )

    let expr = CallExpression {
        token,
        function,
        arguments,
    };

    Some(Box::new(AllExpressions::CallExpression(expr)))
}

pub fn parse_array_literal(p: &mut Parser) -> BoxedExpression {
    let token = p.current_token.clone(); // [

    let elements = parse_comma_sep_arguments(p, &TokenType::Rbracket)?;
    p.next_token(); // consume ]

    let array = AllExpressions::ArrayLiteral(ArrayLiteral { token, elements });
    Some(Box::new(array))
}

pub fn parse_index_expressions(p: &mut Parser, left: BoxedExpression) -> BoxedExpression {
    let token = p.current_token.clone();
    p.next_token(); // consume [

    let index = p.parse_expression(Precedence::Lowest);
    if !p.expect_peek(TokenType::Rbracket) {
        return None;
    }

    let expr = IndexExpression {
        token,
        left: left?,
        index: index?,
    };

    Some(Box::new(AllExpressions::IndexExpression(expr)))
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

fn parse_comma_sep_arguments(p: &mut Parser, end: &TokenType) -> Option<Vec<AllExpressions>> {
    let mut args = Vec::new();

    while !p.peek_token_is(end) {
        p.next_token(); // consumes ( OR ,
        let arg = p.parse_expression(Precedence::Lowest)?;
        args.push(*arg);

        if p.peek_token_is(end) {
            break;
        }

        if !p.expect_peek(TokenType::Comma) {
            return None;
        }
    }

    Some(args)
}

pub fn parse_block_statement(p: &mut Parser) -> BlockStatement {
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
