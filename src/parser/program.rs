use super::tracing::Tracer;
use crate::ast::expressions::AllExpressions;
use crate::ast::program::Program;
use crate::lexer::token::{eof_token, Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::parse_expressions::{
    parse_array_literal, parse_boolean_expression, parse_call_expression, parse_function_literal,
    parse_grouped_expression, parse_hash_literal, parse_identifier, parse_if_expression,
    parse_index_expressions, parse_infix_expression, parse_integer_literal, parse_null_literal,
    parse_prefix_expression, parse_string_literal,
};

/// A type alias for the optional boxed expression type that is commonly used in parser functions
pub type BoxedExpression = Option<Box<AllExpressions>>;

pub type PrefixParseFn = dyn Fn(&mut Parser) -> BoxedExpression;
pub type InfixParseFn = dyn Fn(&mut Parser, BoxedExpression) -> BoxedExpression;

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
            String => Some(Box::new(parse_string_literal)),
            Bang | Minus | Plus => Some(Box::new(parse_prefix_expression)),
            True | False => Some(Box::new(parse_boolean_expression)),
            Lparen => Some(Box::new(parse_grouped_expression)),
            If => Some(Box::new(parse_if_expression)),
            Function => Some(Box::new(parse_function_literal)),
            Lbracket => Some(Box::new(parse_array_literal)),
            Lbrace => Some(Box::new(parse_hash_literal)),
            Null => Some(Box::new(parse_null_literal)),
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
            Lbracket => Some(Box::new(parse_index_expressions)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::test_helpers::*;
    use crate::ast::expressions::AllExpressions;
    use crate::ast::statements::AllStatements;
    use crate::lexer::keywords;

    #[test]
    fn test_let_statements() {
        use Literal::*;

        let tests = vec![
            ("let x = 5", "x", Int(5)),
            ("let y = true;", "y", Bool(true)),
            ("let foobar = y", "foobar", Ident("y")),
            ("let x = \"foobar\"", "x", Str("foobar")),
        ];

        for tc in tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let AllStatements::Let(let_stmt) = program.statements.remove(0) else {
                panic!("{}", EXPECTED_LET);    
            };
            assert_eq!(let_stmt.token.literal, keywords::LET);
            assert_eq!(let_stmt.name.value, tc.1);
            assert_eq!(let_stmt.name.token.literal, tc.1);
            helper_test_literal(tc.2, *let_stmt.value);
        }
    }

    #[test]
    fn test_return_statements() {
        use Literal::*;

        // input, expectedIdent, expectedValue
        let tests = vec![
            ("return 5", Int(5)),
            ("return false;", Bool(false)),
            ("return x", Ident("x")),
        ];

        for tc in tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);

            let AllStatements::Return(return_stmt) = program.statements.remove(0) else {
                panic!("{}", EXPECTED_RETURN);    
            };
            assert_eq!(return_stmt.token.literal, keywords::RETURN);
            helper_test_literal(tc.1, *return_stmt.return_value);
        }

        let mut program = helper_prepare_parser("return x+y;");
        assert_eq!(program.statements.len(), 1);
        let AllStatements::Return(return_stmt) = program.statements.remove(0) else {
            panic!("{}", EXPECTED_RETURN);
        };
        let return_expr = return_stmt.return_value;
        helper_test_infix_expression(*return_expr, Ident("x"), "+", Ident("y"));
    }

    #[test]
    fn test_while_statements() {
        use Literal::*;
        let input = "
            while (1 < 2) {
                let x = 10;
            }
            ";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let AllStatements::While(stmt) = program.statements.remove(0) else {
            panic!("{}", EXPECTED_WHILE);
        };
        assert_eq!(stmt.token.literal, keywords::WHILE);
        helper_test_infix_expression(*stmt.condition, Int(1), "<", Int(2));

        assert_eq!(stmt.body.statements.len(), 1);
    }

    #[test]
    fn test_parse_assignment_expressions() {
        let input = "x = 10;";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let AllExpressions::Assignment(expr) = helper_get_expression(program.statements.remove(0)) else {
             panic!("{}", EXPECTED_ARRAY_LITERAL)
        };

        assert_eq!(expr.ident.value, "x");
        helper_test_integer_literal(&*expr.value, 10);
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
        helper_test_integer_literal(&expr, 5);
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
            let prefix_exp = match helper_get_expression(stmt) {
                AllExpressions::PrefixExpression(v) => v,
                _ => panic!("{}", EXPECTED_PREFIX),
            };

            assert_eq!(prefix_exp.operator, tc.1);
            let right_expr = prefix_exp.right.expect(EXPECTED_RIGHT);

            helper_test_integer_literal(&*right_expr, tc.2);
        }
    }

    type TupleInput<'a> = (&'a str, Literal<'a>, &'a str, Literal<'a>);

    #[test]
    fn test_parsing_infix_expressions() {
        use Literal::*;

        // (input, left_value, operator, right_value)
        let infix_tests: Vec<TupleInput> = vec![
            ("5 + 5;", Int(5_i64), "+", Int(5_i64)),
            ("5 - 5;", Int(5_i64), "-", Int(5_i64)),
            ("5 * 5;", Int(5_i64), "*", Int(5_i64)),
            ("5 / 5;", Int(5_i64), "/", Int(5_i64)),
            ("5 > 5;", Int(5_i64), ">", Int(5_i64)),
            ("5 < 5;", Int(5_i64), "<", Int(5_i64)),
            ("5 == 5;", Int(5_i64), "==", Int(5_i64)),
            ("5 != 5;", Int(5_i64), "!=", Int(5_i64)),
            ("true == true", Bool(true), "==", Bool(true)),
            ("true != false", Bool(true), "!=", Bool(false)),
            ("false == false", Bool(false), "==", Bool(false)),
            ("alice * bob", Ident("alice"), "*", Ident("bob")),
            ("\"foo\" != \"bar\"", Str("foo"), "!=", Str("bar")),
        ];
        for tc in infix_tests {
            let mut program = helper_prepare_parser(tc.0);
            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements.remove(0);
            let expr = helper_get_expression(stmt);
            helper_test_infix_expression(expr, tc.1, tc.2, tc.3);
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
            (
                "a * [1, 2, 3, 4][b * c] * d",
                "((a * ([1, 2, 3, 4][(b * c)])) * d)\n",
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))\n",
            ),
        ];

        for tc in tests {
            let program = helper_prepare_parser(tc.0);
            assert_eq!(tc.1, program.to_string());
        }
    }

    #[test]
    fn test_if_expression() {
        use Literal::Ident;

        let mut program = helper_prepare_parser("if (x < y) { x };");
        assert_eq!(program.statements.len(), 1);

        let mut if_expr = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::IfExpression(v) => v,
            _ => panic!("{}", EXPECTED_IF),
        };
        assert_eq!(if_expr.consequence.statements.len(), 1);

        helper_test_infix_expression(*if_expr.condition, Ident("x"), "<", Ident("y"));

        let consequence = if_expr.consequence.statements.remove(0);
        let consequence_expr = helper_get_expression(consequence);

        helper_test_identifier(consequence_expr, "x");

        assert!(if_expr.alternative.is_none());
    }

    #[test]
    fn test_if_else_expression() {
        use Literal::Ident;

        let mut program = helper_prepare_parser("if (x > y) { x } else { y + z; }");
        assert_eq!(program.statements.len(), 1);

        let mut if_expr = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::IfExpression(v) => v,
            _ => panic!("{}", EXPECTED_IF),
        };
        assert_eq!(if_expr.consequence.statements.len(), 1);

        helper_test_infix_expression(*if_expr.condition, Ident("x"), ">", Ident("y"));

        let consequence = if_expr.consequence.statements.remove(0);
        let consequence_expr = helper_get_expression(consequence);
        helper_test_identifier(consequence_expr, "x");

        assert!(if_expr.alternative.is_some());
        let alternative = if_expr.alternative.unwrap().statements.remove(0);
        let alternative_expr = helper_get_expression(alternative);
        helper_test_infix_expression(
            alternative_expr,
            Literal::Ident("y"),
            "+",
            Literal::Ident("z"),
        );
    }

    #[test]
    fn test_functional_literal() {
        let mut program = helper_prepare_parser("fn(x, y) { x + y; }");
        assert_eq!(program.statements.len(), 1);

        let mut fn_expr = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::FunctionLiteral(v) => v,
            _ => panic!("{}", EXPECTED_FUNC),
        };
        assert_eq!(fn_expr.parameters.len(), 2);

        assert_eq!(fn_expr.parameters[0].value, "x");
        assert_eq!(fn_expr.parameters[1].value, "y");

        assert_eq!(fn_expr.body.statements.len(), 1);
        let stmt = fn_expr.body.statements.remove(0);
        let body_expr = helper_get_expression(stmt);
        helper_test_infix_expression(body_expr, Literal::Ident("x"), "+", Literal::Ident("y"));
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
            let fn_expr = match helper_get_expression(program.statements.remove(0)) {
                AllExpressions::FunctionLiteral(v) => v,
                _ => panic!("{}", EXPECTED_FUNC),
            };
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

        let call_expr = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::CallExpression(v) => v,
            _ => panic!("{}", EXPECTED_CALL),
        };

        helper_test_identifier(*call_expr.function, "add");

        let mut args = call_expr.arguments;
        assert_eq!(args.len(), 4);
        helper_test_integer_literal(&args.remove(0), 1);
        helper_test_infix_expression(args.remove(0), Literal::Int(2), "*", Literal::Int(3));
        helper_test_infix_expression(args.remove(0), Literal::Int(4), "+", Literal::Int(5));
        helper_test_identifier(args.remove(0), "x");

        let input = "print();";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);
        let call_expr = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::CallExpression(v) => v,
            _ => panic!("{}", EXPECTED_CALL),
        };
        helper_test_identifier(*call_expr.function, "print");
        assert_eq!(call_expr.arguments.len(), 0);
    }

    #[test]
    fn test_parse_array_literals() {
        use Literal::Int;

        let input = "[1, 2 * 3, 3 + 3]";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let mut array = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::ArrayLiteral(v) => v,
            _ => panic!("{}", EXPECTED_ARRAY_LITERAL),
        };

        assert_eq!(array.elements.len(), 3);
        helper_test_integer_literal(&array.elements.remove(0), 1);
        helper_test_infix_expression(array.elements.remove(0), Int(2), "*", Int(3));
        helper_test_infix_expression(array.elements.remove(0), Int(3), "+", Int(3));

        // asserting empty array
        let input = "[]";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let array = match helper_get_expression(program.statements.remove(0)) {
            AllExpressions::ArrayLiteral(v) => v,
            _ => panic!("{}", EXPECTED_ARRAY_LITERAL),
        };
        assert!(array.elements.is_empty());
    }

    #[test]
    fn test_parse_index_expressions() {
        use Literal::Int;
        let input = "myArray[1 + 1]";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let AllExpressions::IndexExpression(expr) = helper_get_expression(program.statements.remove(0)) else {
             panic!("{}", EXPECTED_INDEX_EXPRESSION);
        };

        helper_test_identifier(*expr.left, "myArray");
        helper_test_infix_expression(*expr.index, Int(1), "+", Int(1));
    }

    #[test]
    fn test_parse_range_expressions() {
        use Literal::{Ident, Int};
        let input = "myArray[1+1 : x-20]";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let AllExpressions::RangeExpression(expr) = helper_get_expression(program.statements.remove(0)) else {
             panic!("{}", EXPECTED_RANGE_EXPRESSION);
        };

        helper_test_identifier(*expr.left, "myArray");
        helper_test_infix_expression(*expr.left_index, Int(1), "+", Int(1));
        helper_test_infix_expression(*expr.right_index, Ident("x"), "-", Int(20));
    }

    #[test]
    fn test_parse_hash_literal() {
        let input = r#"{"one": 1, "two": 2, "three": 3}"#;
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);

        let AllExpressions::HashLiteral(expr) = helper_get_expression(program.statements.remove(0)) else {
            panic!("{}", EXPECTED_HASH_LITERAL);
        };
        assert_eq!(expr.pairs.len(), 3);

        let expected = HashMap::from([("one", 1), ("two", 2), ("three", 3)]);

        for entry in expr.pairs.iter() {
            let AllExpressions::StringLiteral(key) = entry.0 else {
                panic!("{}", EXPECTED_STRING);
            };
            let expected_int = expected.get(key.token.literal.as_str()).unwrap();
            helper_test_integer_literal(entry.1, *expected_int);
        }

        // assert empty hash literal working correctly
        let input = "{}";
        let mut program = helper_prepare_parser(input);
        assert_eq!(program.statements.len(), 1);
        let AllExpressions::HashLiteral(expr) = helper_get_expression(program.statements.remove(0)) else {
            panic!("{}", EXPECTED_HASH_LITERAL);
        };
        assert_eq!(expr.pairs.len(), 0);
    }
}

/// Contains helper functions and constants useful for testing parsing
#[cfg(test)]
mod test_helpers {
    use super::{Lexer, Parser};
    use crate::ast::expressions::AllExpressions;
    use crate::ast::program::Program;
    use crate::ast::statements::AllStatements;

    pub enum Literal<'a> {
        Int(i64),
        Bool(bool),
        Ident(&'a str),
        Str(&'a str),
    }

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

    pub fn helper_test_integer_literal(expr: &AllExpressions, value: i64) {
        let AllExpressions::IntegerLiteral(integer_literal) = expr else {
            panic!("{}", EXPECTED_INTEGER);
        };
        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token.literal, format!("{}", value));
    }

    pub fn helper_test_string_literal(expr: AllExpressions, value: &str) {
        let AllExpressions::StringLiteral(str_literal) = expr else {
            panic!("{}", EXPECTED_STRING);
        };
        assert_eq!(str_literal.token.literal, value);
    }

    pub fn helper_test_identifier(expr: AllExpressions, value: &str) {
        let AllExpressions::Identifier(identifier) = expr else {
            panic!("{}", EXPECTED_IDENT);
        };
        assert_eq!(identifier.value, value);
        assert_eq!(identifier.token.literal, format!("{}", value));
    }

    pub fn helper_test_boolean_literal(expr: AllExpressions, value: bool) {
        let AllExpressions::Boolean(boolean) = expr else {
            panic!("{}", EXPECTED_BOOLEAN);
        };
        assert_eq!(boolean.value, value);
        assert_eq!(boolean.token.literal, value.to_string());
    }

    pub fn helper_test_infix_expression(
        expr: AllExpressions,
        left: Literal,
        operator: &str,
        right: Literal,
    ) {
        let AllExpressions::InfixExpression(infix_expr) = expr else {
            panic!("{}", EXPECTED_INFIX);
        };
        helper_test_literal(left, *infix_expr.left.expect(EXPECTED_LEFT));
        assert_eq!(infix_expr.operator, operator);
        helper_test_literal(right, *infix_expr.right.expect(EXPECTED_RIGHT));
    }

    pub fn helper_prepare_parser(input: &str) -> Program {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        helper_check_parser_errors(&p.errors);
        program
    }

    pub fn helper_get_expression(stmt: AllStatements) -> AllExpressions {
        let AllStatements::Expression(expr_stmt) = stmt else {
            panic!("{}", EXPECTED_EXPRESSION_STATEMENT); 
        };
        return *expr_stmt.expression.expect(EXPECTED_EXPRESSION);
    }

    pub fn helper_test_literal(expected: Literal, expr: AllExpressions) {
        match expected {
            Literal::Int(val) => helper_test_integer_literal(&expr, val),
            Literal::Bool(val) => helper_test_boolean_literal(expr, val),
            Literal::Ident(val) => helper_test_identifier(expr, val),
            Literal::Str(val) => helper_test_string_literal(expr, val),
        }
    }

    pub const EXPECTED_IDENT: &str = "expected an identifier";
    pub const EXPECTED_LET: &str = "expected a let statement";
    pub const EXPECTED_RETURN: &str = "expected a return statement";
    pub const EXPECTED_WHILE: &str = "expected a while statement";
    pub const EXPECTED_INTEGER: &str = "expected an integer literal";
    pub const EXPECTED_STRING: &str = "expected a string literal";
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
    pub const EXPECTED_ARRAY_LITERAL: &str = "expected an array literal";
    pub const EXPECTED_INDEX_EXPRESSION: &str = "expected an array index expression";
    pub const EXPECTED_RANGE_EXPRESSION: &str = "expected an array index range expression";
    pub const EXPECTED_HASH_LITERAL: &str = "expected a hash literal";
}
