use super::parse_expressions::{
    parse_boolean_expression, parse_identifier, parse_infix_expression, parse_integer_literal,
    parse_prefix_expression,
};

use super::tracing::Tracer;
use crate::ast::program::Program;
use crate::ast::Expression;
use crate::lexer::token::{eof_token, Token, TokenType};
use crate::lexer::Lexer;

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
    pub fn prefix_parse_function(token_type: TokenType) -> Option<Box<PrefixParseFn>> {
        use TokenType::*;

        match token_type {
            Ident => Some(Box::new(parse_identifier)),
            Int => Some(Box::new(parse_integer_literal)),
            Bang | Minus | Plus => Some(Box::new(parse_prefix_expression)),
            True | False => Some(Box::new(parse_boolean_expression)),
            _ => None,
        }
    }

    /// Returns the corresponding infix parse function
    pub fn infix_parse_function(token_type: TokenType) -> Option<Box<InfixParseFn>> {
        use TokenType::*;

        match token_type {
            Plus | Minus | Asterisk | Slash | Eq | NotEq | Lt | Gt => {
                Some(Box::new(parse_infix_expression))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Parser};
    use crate::ast::expressions::{
        Boolean, Identifier, InfixExpression, IntegerLiteral, PrefixExpression,
    };
    use crate::ast::program::Program;
    use crate::ast::statements::{ExpressionStatement, LetStatement, ReturnStatement};
    use crate::ast::{Expression, Node, Statement};
    use crate::lexer::keywords;
    use std::any::{Any, TypeId};

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
        let y = 10;
        let foobar = 838383;";

        let program = prepare_parser(input);
        assert_eq!(program.statements.len(), 3);

        let test_cases = vec!["x", "y", "foobar"];

        for (i, stmt) in program.statements.into_iter().enumerate() {
            let expected_name = test_cases[i];

            let let_stmt = match stmt.into_any().downcast::<LetStatement>() {
                Ok(val) => val,
                Err(e) => panic!("expected a let statement, found {:?}", e),
            };

            test_let_statement(*let_stmt, expected_name);
        }
    }

    fn test_let_statement(s: LetStatement, name: &str) {
        assert_eq!(s.token_literal(), keywords::LET);
        assert_eq!(s.name.value, name);
        assert_eq!(s.name.token_literal(), name);
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;
        return 10;
        return 993322;";

        let program = prepare_parser(input);
        assert_eq!(program.statements.len(), 3);

        for stmt in program.statements.into_iter() {
            let return_stmt = match stmt.into_any().downcast::<ReturnStatement>() {
                Ok(v) => v,
                Err(e) => panic!("expected a return statement, found {:?}", e),
            };
            assert_eq!(return_stmt.token_literal(), keywords::RETURN);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let program = prepare_parser("foobar;");
        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements.into_iter() {
            let expr_any = get_expression_any_statement(stmt);
            let identifier = match expr_any.downcast::<Identifier>() {
                Ok(v) => v,
                Err(e) => panic!("expected an identifier statement, found {:?}", e),
            };

            assert_eq!(identifier.value, "foobar");
            assert_eq!(identifier.token_literal(), "foobar");
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let program = prepare_parser("5;");
        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements.into_iter() {
            let integer_literal =
                match get_expression_any_statement(stmt).downcast::<IntegerLiteral>() {
                    Ok(v) => v,
                    Err(e) => panic!("expected an integer literal expression, found {:?}", e),
                };

            assert_eq!(integer_literal.value, 5);
            assert_eq!(integer_literal.token_literal(), "5");
        }
    }

    #[test]
    fn test_boolean_expression_statement() {
        let program = prepare_parser("true;");
        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements.into_iter() {
            let boolean = match get_expression_any_statement(stmt).downcast::<Boolean>() {
                Ok(v) => v,
                Err(e) => panic!("expected a boolean expression, found {:?}", e),
            };

            assert_eq!(boolean.value, true);
            assert_eq!(boolean.token_literal(), "true");
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        // (input, operator, integer_value)
        let prefix_tests = vec![("!5", "!", 5), ("-15", "-", 15)];

        for tc in prefix_tests {
            let mut program = prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements.remove(0);
            let prefix_exp = match get_expression_any_statement(stmt).downcast::<PrefixExpression>()
            {
                Ok(v) => v,
                Err(e) => panic!("expected an integer literal statement, found {:?}", e),
            };

            assert_eq!(prefix_exp.operator, tc.1);
            let right_expr = prefix_exp.right.expect("right expression should exist");

            test_integer_literal(right_expr, tc.2);
        }
    }

    type TupleInput<'a> = (&'a str, Box<dyn Any>, &'a str, Box<dyn Any>);

    #[test]
    fn test_parsing_infix_expressions() {
        // (input, left_value, operator, right_value)
        let infix_tests: Vec<TupleInput> = vec![
            ("5 + 5;", Box::new(5), "+", Box::new(5)),
            ("5 - 5;", Box::new(5), "-", Box::new(5)),
            ("5 * 5;", Box::new(5), "*", Box::new(5)),
            ("5 / 5;", Box::new(5), "/", Box::new(5)),
            ("5 > 5;", Box::new(5), ">", Box::new(5)),
            ("5 < 5;", Box::new(5), "<", Box::new(5)),
            ("5 == 5;", Box::new(5), "==", Box::new(5)),
            ("5 != 5;", Box::new(5), "!=", Box::new(5)),
            ("true == true", Box::new(true), "==", Box::new(true)),
            ("true != false", Box::new(true), "!=", Box::new(false)),
            ("false == false", Box::new(false), "==", Box::new(false)),
        ];
        for tc in infix_tests {
            parse_infix_expression(tc);
        }
    }

    fn parse_infix_expression(tc: TupleInput) {
        let mut program = prepare_parser(tc.0);
        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements.remove(0);
        let infix_expr = match get_expression_any_statement(stmt).downcast::<InfixExpression>() {
            Ok(v) => v,
            Err(e) => panic!("expected an infix expression statement, found {:?}", e),
        };

        if TypeId::of::<i64>() == tc.1.type_id() {
            let l = tc.1.downcast::<i64>().unwrap();
            test_integer_literal(infix_expr.left.expect("left expression should exist"), *l);
        } else if TypeId::of::<bool>() == tc.1.type_id() {
            let l = tc.1.downcast::<bool>().unwrap();
            test_boolean_literal(infix_expr.left.expect("left expression should exist"), *l);
        }

        assert_eq!(infix_expr.operator, tc.2);

        if TypeId::of::<i64>() == tc.3.type_id() {
            let r = tc.3.downcast::<i64>().unwrap();
            test_integer_literal(infix_expr.right.expect("right expression should exist"), *r);
        } else if TypeId::of::<bool>() == tc.3.type_id() {
            let r = tc.3.downcast::<bool>().unwrap();
            test_boolean_literal(infix_expr.right.expect("left expression should exist"), *r);
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
        ];

        for tc in tests {
            let program = prepare_parser(tc.0);
            assert_eq!(tc.1, program.to_string());
        }
    }

    fn check_parser_errors(errors: &Vec<String>) {
        if errors.len() == 0 {
            return;
        }

        let mut err_msg = String::new();
        for msg in errors {
            err_msg.push_str(format!("\tparser error: {}\n", msg).as_str());
        }

        panic!("parser has {} errors\n{}", errors.len(), err_msg);
    }

    fn test_integer_literal(expr: Box<dyn Expression>, value: i64) {
        let integer_literal = match expr.into_any().downcast::<IntegerLiteral>() {
            Ok(v) => v,
            Err(e) => panic!("expected an integer literal expression, found {:?}", e),
        };
        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token_literal(), format!("{}", value));
    }

    fn test_boolean_literal(expr: Box<dyn Expression>, value: bool) {
        let boolean = match expr.into_any().downcast::<Boolean>() {
            Ok(v) => v,
            Err(e) => panic!("expected a boolean expression, found {:?}", e),
        };

        assert_eq!(boolean.value, true);
        assert_eq!(boolean.token_literal(), value.to_string());
    }

    fn prepare_parser(input: &str) -> Program {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p.errors);
        program
    }

    fn get_expression_any_statement(stmt: Box<dyn Statement>) -> Box<dyn Any> {
        let expr_stmt = match stmt.into_any().downcast::<ExpressionStatement>() {
            Ok(v) => v,
            Err(e) => panic!("expected an expression statement, found {:?}", e),
        };
        expr_stmt
            .expression
            .expect("expected the expression of the expr_statement to exist")
            .into_any()
    }
}
