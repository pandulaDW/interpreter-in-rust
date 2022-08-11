use std::collections::HashMap;

use super::parse_expressions::{
    parse_identifier, parse_infix_expression, parse_integer_literal, parse_prefix_expression,
};
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

    pub current_token: Token,
    pub peek_token: Token,

    pub prefix_parse_fns: HashMap<TokenType, Box<PrefixParseFn>>,
    pub infix_parse_fns: HashMap<TokenType, Box<InfixParseFn>>,
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
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Read two tokens, so curToken and peekToken are both set
        p.next_token();
        p.next_token();

        // register the expression parsers
        p.register_prefix(TokenType::Ident, Box::new(parse_identifier));
        p.register_prefix(TokenType::Int, Box::new(parse_integer_literal));
        p.register_prefix(TokenType::Bang, Box::new(parse_prefix_expression));
        p.register_prefix(TokenType::Minus, Box::new(parse_prefix_expression));
        p.register_infix(TokenType::Plus, Box::new(parse_infix_expression));
        p.register_infix(TokenType::Minus, Box::new(parse_infix_expression));
        p.register_infix(TokenType::Asterisk, Box::new(parse_infix_expression));
        p.register_infix(TokenType::Slash, Box::new(parse_infix_expression));
        p.register_infix(TokenType::Eq, Box::new(parse_infix_expression));
        p.register_infix(TokenType::NotEq, Box::new(parse_infix_expression));
        p.register_infix(TokenType::Lt, Box::new(parse_infix_expression));
        p.register_infix(TokenType::Gt, Box::new(parse_infix_expression));

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
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Parser};
    use crate::ast::expressions::{Identifier, InfixExpression, IntegerLiteral, PrefixExpression};
    use crate::ast::program::Program;
    use crate::ast::statements::{ExpressionStatement, LetStatement, ReturnStatement};
    use crate::ast::{Expression, Node, Statement};
    use crate::lexer::keywords;
    use std::any::Any;

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

    #[test]
    fn test_parsing_infix_expressions() {
        // (input, left_value, operator, right_value)
        let infix_tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for tc in infix_tests {
            let mut program = prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements.remove(0);
            let infix_expr = match get_expression_any_statement(stmt).downcast::<InfixExpression>()
            {
                Ok(v) => v,
                Err(e) => panic!("expected an infix expression statement, found {:?}", e),
            };

            test_integer_literal(infix_expr.left.expect("left expression should exist"), tc.1);
            assert_eq!(infix_expr.operator, tc.2);
            test_integer_literal(
                infix_expr.right.expect("right expression should exist"),
                tc.3,
            );
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        // input, expected
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
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
