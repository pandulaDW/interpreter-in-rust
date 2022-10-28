use super::parse_expressions::{
    parse_boolean_expression, parse_grouped_expression, parse_identifier, parse_infix_expression,
    parse_integer_literal, parse_prefix_expression,
};

use super::tracing::Tracer;
use crate::ast::program::Program;
use crate::ast::Expression;
use crate::lexer::token::{eof_token, Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::parse_expressions::{
    parse_call_expression, parse_function_literal, parse_if_expression,
};

pub type PrefixParseFn = dyn Fn(&mut Parser) -> Option<Box<dyn Expression>>;
pub type InfixParseFn =
    dyn Fn(&mut Parser, Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>>;

/// Parser represents the main structure which advances the lexer and parses the tokens as needed
/// into AST statements.
///
/// It includes the information needed for parsing as well as parser results
pub struct Parser {
    pub l: Lexer,
    pub errors: Vec<String>,
    pub tracer: Tracer,

    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    /// Returns a new parser using the provided lexer.
    ///
    /// It also reads the two initial tokens
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            current_token: eof_token(),
            peek_token: eof_token(),
            tracer: Tracer::new(),
            errors: vec![],
        };

        // Read two tokens, so curToken and peekToken are both set
        p.next_token();
        p.next_token();

        p
    }

    /// The main parser method, which iterates through the tokens and generates a list of AST statements
    /// which ships with the `Program`
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.token_type != TokenType::Eof {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                program.statements.push(s);
            }
            self.next_token();
        }

        program
    }

    /// Returns the corresponding prefix parse function
    pub fn prefix_parse_function(token_type: &TokenType) -> Option<Box<PrefixParseFn>> {
        use TokenType::*;

        match token_type {
            Ident => Some(Box::new(parse_identifier)),
            Int => Some(Box::new(parse_integer_literal)),
            Bang | Minus | Plus => Some(Box::new(parse_prefix_expression)),
            True | False => Some(Box::new(parse_boolean_expression)),
            Lparen => Some(Box::new(parse_grouped_expression)),
            If => Some(Box::new(parse_if_expression)),
            Function => Some(Box::new(parse_function_literal)),
            _ => None,
        }
    }

    /// Returns the corresponding infix parse function
    pub fn infix_parse_function(token_type: &TokenType) -> Option<Box<InfixParseFn>> {
        use TokenType::*;

        match token_type {
            Plus | Minus | Asterisk | Slash | Eq | NotEq | Lt | Gt => {
                Some(Box::new(parse_infix_expression))
            }
            Lparen => Some(Box::new(parse_call_expression)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use crate::ast::expressions::{
        CallExpression, FunctionLiteral, Identifier, IfExpression, PrefixExpression,
    };
    use crate::ast::statements::AllStatements;
    use crate::ast::Node;
    use crate::lexer::keywords;
    use std::any::Any;

    #[test]
    fn test_let_statements() {
        // input, expectedIdent, expectedValue
        let tests: Vec<(&str, &str, Box<dyn Any>)> = vec![
            ("let x = 5", "x", Box::new(5)),
            ("let y = true;", "y", Box::new(true)),
            ("let foobar = y", "foobar", Box::new("y")),
        ];

        for tc in tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);

            if let AllStatements::Let(let_stmt) = program.statements.remove(0) {
                assert_eq!(let_stmt.token_literal(), keywords::LET);
                assert_eq!(let_stmt.name.value, tc.1);
                assert_eq!(let_stmt.name.token_literal(), tc.1);
                helper_test_literal(tc.2, let_stmt.value.expect(EXPECTED_EXPRESSION));
            } else {
                panic!("{}", EXPECTED_LET);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        // input, expectedIdent, expectedValue
        let tests: Vec<(&str, Box<dyn Any>)> = vec![
            ("return 5", Box::new(5)),
            ("return false;", Box::new(false)),
            ("return x", Box::new("x")),
        ];

        for tc in tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);

            if let AllStatements::Return(return_stmt) = program.statements.remove(0) {
                assert_eq!(return_stmt.token_literal(), keywords::RETURN);
                helper_test_literal(tc.1, return_stmt.return_value.expect(EXPECTED_EXPRESSION));
            } else {
                panic!("{}", EXPECTED_RETURN);
            }
        }

        let mut program = helper_prepare_parser("return x+y;");
        assert_eq!(program.statements.len(), 1);
        if let AllStatements::Return(return_stmt) = program.statements.remove(0) {
            let return_expr = return_stmt.return_value.expect(EXPECTED_EXPRESSION);
            helper_test_infix_expression(return_expr.into_any(), Box::new("x"), "+", Box::new("y"));
        } else {
            panic!("{}", EXPECTED_RETURN);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let mut program = helper_prepare_parser("foobar;");
        assert_eq!(program.statements.len(), 1);

        let expr = helper_get_expression(program.statements.remove(0));
        helper_test_identifier(expr, "foobar");
    }

    #[test]
    fn test_integer_literal_expression() {
        let mut program = helper_prepare_parser("5;");
        assert_eq!(program.statements.len(), 1);

        let expr = helper_get_expression(program.statements.remove(0));
        helper_test_integer_literal(expr, 5);
    }

    #[test]
    fn test_boolean_expression_statement() {
        let mut program = helper_prepare_parser("true;");
        assert_eq!(program.statements.len(), 1);

        let expr = helper_get_expression(program.statements.remove(0));
        helper_test_boolean_literal(expr, true);
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        // (input, operator, integer_value)
        let prefix_tests = vec![("!5", "!", 5), ("-15", "-", 15)];

        for tc in prefix_tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements.remove(0);
            let prefix_exp = helper_get_expression_any(stmt)
                .downcast::<PrefixExpression>()
                .expect(EXPECTED_PREFIX);

            assert_eq!(prefix_exp.operator, tc.1);
            let right_expr = prefix_exp.right.expect(EXPECTED_RIGHT);

            helper_test_integer_literal(right_expr, tc.2);
        }
    }

    type TupleInput<'a> = (&'a str, Box<dyn Any>, &'a str, Box<dyn Any>);

    #[test]
    fn test_parsing_infix_expressions() {
        // (input, left_value, operator, right_value)
        let infix_tests: Vec<TupleInput> = vec![
            ("5 + 5;", Box::new(5_i64), "+", Box::new(5_i64)),
            ("5 - 5;", Box::new(5_i64), "-", Box::new(5_i64)),
            ("5 * 5;", Box::new(5_i64), "*", Box::new(5_i64)),
            ("5 / 5;", Box::new(5_i64), "/", Box::new(5_i64)),
            ("5 > 5;", Box::new(5_i64), ">", Box::new(5_i64)),
            ("5 < 5;", Box::new(5_i64), "<", Box::new(5_i64)),
            ("5 == 5;", Box::new(5_i64), "==", Box::new(5_i64)),
            ("5 != 5;", Box::new(5_i64), "!=", Box::new(5_i64)),
            ("true == true", Box::new(true), "==", Box::new(true)),
            ("true != false", Box::new(true), "!=", Box::new(false)),
            ("false == false", Box::new(false), "==", Box::new(false)),
            ("alice * bob", Box::new("alice"), "*", Box::new("bob")),
        ];
        for tc in infix_tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements.remove(0);
            let expr_any = helper_get_expression_any(stmt);
            helper_test_infix_expression(expr_any, tc.1, tc.2, tc.3);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        // input, expected
        let tests = vec![
            ("-a * b", "((-a) * b)\n"),
            ("!-a", "(!(-a))\n"),
            ("a + b + c", "((a + b) + c)\n"),
            ("a + b - c", "((a + b) - c)\n"),
            ("a * b * c", "((a * b) * c)\n"),
            ("a * b * c", "((a * b) * c)\n"),
            ("a * b / c", "((a * b) / c)\n"),
            ("a + b / c", "(a + (b / c))\n"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)\n"),
            ("3 + 4; -5 * 5", "(3 + 4)\n((-5) * 5)\n"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))\n"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))\n"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))\n",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))\n",
            ),
            ("true", "true\n"),
            ("false", "false\n"),
            ("3 > 5 == false", "((3 > 5) == false)\n"),
            ("3 < 5 == true", "((3 < 5) == true)\n"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)\n"),
            ("(5 + 5) * 2", "((5 + 5) * 2)\n"),
            ("2 / (5 + 5)", "(2 / (5 + 5))\n"),
            ("-(5 + 5)", "(-(5 + 5))\n"),
            ("!(true == true)", "(!(true == true))\n"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)\n"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))\n",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))\n",
            ),
        ];

        for tc in tests {
            let program = helper_prepare_parser(tc.0);
            assert_eq!(tc.1, program.to_string());
        }
    }

    #[test]
    fn test_if_expression() {
        let mut program = helper_prepare_parser("if (x < y) { x };");
        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.remove(0);
        let mut if_expr = helper_get_expression_any(stmt)
            .downcast::<IfExpression>()
            .expect(EXPECTED_IF);
        assert_eq!(if_expr.consequence.statements.len(), 1);

        let condition = if_expr.condition.into_any();
        helper_test_infix_expression(condition, Box::new("x"), "<", Box::new("y"));

        let consequence = if_expr.consequence.statements.remove(0);
        let consequence_expr = helper_get_expression_any(consequence)
            .downcast::<Identifier>()
            .expect(EXPECTED_IDENT);
        helper_test_identifier(consequence_expr, "x");

        assert!(if_expr.alternative.is_none());
    }

    #[test]
    fn test_if_else_expression() {
        let mut program = helper_prepare_parser("if (x > y) { x } else { y + z; }");
        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.remove(0);
        let mut if_expr = helper_get_expression_any(stmt)
            .downcast::<IfExpression>()
            .expect(EXPECTED_IF);
        assert_eq!(if_expr.consequence.statements.len(), 1);

        let condition = if_expr.condition.into_any();
        helper_test_infix_expression(condition, Box::new("x"), ">", Box::new("y"));

        let consequence = if_expr.consequence.statements.remove(0);
        let consequence_expr = helper_get_expression_any(consequence)
            .downcast::<Identifier>()
            .expect(EXPECTED_IDENT);
        helper_test_identifier(consequence_expr, "x");

        assert!(if_expr.alternative.is_some());
        let alternative = if_expr.alternative.unwrap().statements.remove(0);
        let alternative_expr = helper_get_expression_any(alternative);
        helper_test_infix_expression(alternative_expr, Box::new("y"), "+", Box::new("z"));
    }

    #[test]
    fn test_functional_literal() {
        let mut program = helper_prepare_parser("fn(x, y) { x + y; }");
        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.remove(0);
        let mut fn_expr = helper_get_expression_any(stmt)
            .downcast::<FunctionLiteral>()
            .expect(EXPECTED_FUNC);
        assert_eq!(fn_expr.parameters.len(), 2);

        assert_eq!(fn_expr.parameters[0].value, "x");
        assert_eq!(fn_expr.parameters[1].value, "y");

        assert_eq!(fn_expr.body.statements.len(), 1);
        let stmt = fn_expr.body.statements.remove(0);
        let body_expr = helper_get_expression_any(stmt);
        helper_test_infix_expression(body_expr, Box::new("x"), "+", Box::new("y"));
    }

    #[test]
    fn test_parse_fn_literal_parameters() {
        // (input, expected_params)
        let input = [
            ("fn() {};", vec![]),
            ("fn(x,y,z) {}", vec!["x", "y", "z"]),
            ("fn(x){}", vec!["x"]),
        ];

        for tc in input {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let fn_expr = helper_get_expression_any(program.statements.remove(0))
                .downcast::<FunctionLiteral>()
                .expect(EXPECTED_FUNC);
            assert_eq!(fn_expr.parameters.len(), tc.1.len());

            tc.1.into_iter()
                .enumerate()
                .for_each(|(i, param)| assert_eq!(fn_expr.parameters[i].value, param));
        }
    }

    #[test]
    fn test_parse_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5, x);";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.remove(0);
        let call_expr = helper_get_expression_any(stmt)
            .downcast::<CallExpression>()
            .expect(EXPECTED_CALL);
        helper_test_identifier(call_expr.function, "add");

        let mut args = call_expr.arguments;
        assert_eq!(args.len(), 4);
        helper_test_integer_literal(args.remove(0), 1);
        helper_test_infix_expression(args.remove(0).into_any(), Box::new(2), "*", Box::new(3));
        helper_test_infix_expression(args.remove(0).into_any(), Box::new(4), "+", Box::new(5));
        helper_test_identifier(args.remove(0), "x");

        let input = "print();";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);
        let call_expr = helper_get_expression_any(program.statements.remove(0))
            .downcast::<CallExpression>()
            .unwrap();
        helper_test_identifier(call_expr.function, "print");
        assert_eq!(call_expr.arguments.len(), 0);
    }
}

/// Contains helper functions and constants useful for testing parsing
#[cfg(test)]
mod test_helpers {
    use super::{Lexer, Parser};
    use crate::ast::expressions::{Boolean, Identifier, InfixExpression, IntegerLiteral};
    use crate::ast::program::Program;
    use crate::ast::statements::AllStatements;
    use crate::ast::{Expression, Node};
    use std::any::{Any, TypeId};

    pub fn helper_check_parser_errors(errors: &Vec<String>) {
        if errors.is_empty() {
            return;
        }

        let mut err_msg = String::new();
        for msg in errors {
            err_msg.push_str(format!("\tparser error: {}\n", msg).as_str());
        }

        panic!("parser has {} error(s)\n{}", errors.len(), err_msg);
    }

    pub fn helper_test_integer_literal(expr: Box<dyn Expression>, value: i64) {
        let integer_literal = expr
            .into_any()
            .downcast::<IntegerLiteral>()
            .expect(EXPECTED_INTEGER);
        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token_literal(), format!("{}", value));
    }

    pub fn helper_test_identifier(expr: Box<dyn Expression>, value: &str) {
        let identifier = expr
            .into_any()
            .downcast::<Identifier>()
            .expect(EXPECTED_IDENT);
        assert_eq!(identifier.value, value);
        assert_eq!(identifier.token_literal(), format!("{}", value));
    }

    pub fn helper_test_boolean_literal(expr: Box<dyn Expression>, value: bool) {
        let boolean = expr
            .into_any()
            .downcast::<Boolean>()
            .expect(EXPECTED_BOOLEAN);

        assert_eq!(boolean.value, value);
        assert_eq!(boolean.token_literal(), value.to_string());
    }

    pub fn helper_test_infix_expression(
        expr_any: Box<dyn Any>,
        left: Box<dyn Any>,
        operator: &str,
        right: Box<dyn Any>,
    ) {
        let infix_expr = expr_any
            .downcast::<InfixExpression>()
            .expect(EXPECTED_INFIX);

        helper_test_literal(left, infix_expr.left.expect(EXPECTED_LEFT));
        assert_eq!(infix_expr.operator, operator);
        helper_test_literal(right, infix_expr.right.expect(EXPECTED_RIGHT));
    }

    pub fn helper_prepare_parser(input: &str) -> Program {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        helper_check_parser_errors(&p.errors);
        program
    }

    pub fn helper_get_expression_any(stmt: AllStatements) -> Box<dyn Any> {
        if let AllStatements::Expression(expr_stmt) = stmt {
            return expr_stmt.expression.expect(EXPECTED_EXPRESSION).into_any();
        } else {
            panic!("{}", EXPECTED_EXPRESSION_STATEMENT);
        }
    }

    pub fn helper_get_expression(stmt: AllStatements) -> Box<dyn Expression> {
        if let AllStatements::Expression(expr_stmt) = stmt {
            return expr_stmt.expression.expect(EXPECTED_EXPRESSION);
        } else {
            panic!("{}", EXPECTED_EXPRESSION_STATEMENT);
        }
    }

    pub fn helper_test_literal(expected: Box<dyn Any>, actual: Box<dyn Expression>) {
        let expected_type_id = (&*expected).type_id();

        if TypeId::of::<i64>() == expected_type_id {
            let val = expected.downcast::<i64>().unwrap();
            helper_test_integer_literal(actual, *val);
        } else if TypeId::of::<bool>() == expected_type_id {
            let val = expected.downcast::<bool>().unwrap();
            helper_test_boolean_literal(actual, *val);
        } else if TypeId::of::<&str>() == expected_type_id {
            let val = expected.downcast::<&str>().unwrap();
            helper_test_identifier(actual, *val);
        }
    }

    pub const EXPECTED_IDENT: &str = "expected an identifier";
    pub const EXPECTED_LET: &str = "expected a let statement";
    pub const EXPECTED_RETURN: &str = "expected a return statement";
    pub const EXPECTED_INTEGER: &str = "expected an integer literal";
    pub const EXPECTED_BOOLEAN: &str = "expected a boolean expression";
    pub const EXPECTED_PREFIX: &str = "expected a prefix expression";
    pub const EXPECTED_INFIX: &str = "expected an infix expression";
    pub const EXPECTED_IF: &str = "expected an if expression";
    pub const EXPECTED_FUNC: &str = "expected an function literal expression";
    pub const EXPECTED_CALL: &str = "expected a call expression";
    pub const EXPECTED_LEFT: &str = "expected the left expression to exist";
    pub const EXPECTED_RIGHT: &str = "expected the right expression to exist";
    pub const EXPECTED_EXPRESSION_STATEMENT: &str = "expected an expression statement";
    pub const EXPECTED_EXPRESSION: &str = "expected an expression";
}
