#![allow(dead_code)]

use std::collections::HashMap;

use super::parse_expressions::parse_identifier;
use crate::ast::program::Program;
use crate::ast::Expression;
use crate::lexer::token::{eof_token, Token, TokenType};
use crate::lexer::Lexer;

pub type PrefixParseFn = dyn Fn(&mut Parser) -> Box<dyn Expression>;
pub type InfixParseFn = dyn Fn(Box<dyn Expression>) -> Box<dyn Expression>;

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
    use crate::ast::expressions::Identifier;
    use crate::ast::statements::{ExpressionStatement, LetStatement, ReturnStatement};
    use crate::ast::Node;
    use crate::lexer::keywords;

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
        let y = 10;
        let foobar = 838383;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert_eq!(program.statements.len(), 3);

        check_parser_errors(&p.errors);

        let test_cases = vec!["x", "y", "foobar"];

        for (i, stmt) in program.statements.into_iter().enumerate() {
            let stmt_any = stmt.into_any();
            let expected_name = test_cases[i];

            let let_stmt = match stmt_any.downcast::<LetStatement>() {
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

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert_eq!(program.statements.len(), 3);
        check_parser_errors(&p.errors);

        for stmt in program.statements.into_iter() {
            let stmt_any = stmt.into_any();
            let return_stmt = match stmt_any.downcast::<ReturnStatement>() {
                Ok(v) => v,
                Err(e) => panic!("expected a return statement, found {:?}", e),
            };
            assert_eq!(return_stmt.token_literal(), keywords::RETURN);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        check_parser_errors(&p.errors);
        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements.into_iter() {
            let stmt_any = stmt.into_any();
            let expr_stmt = match stmt_any.downcast::<ExpressionStatement>() {
                Ok(v) => v,
                Err(e) => panic!("expected an expression statement, found {:?}", e),
            };

            let expression = expr_stmt
                .expression
                .expect("expected the expression of the expr_statement to exist");

            let expr_any = expression.into_any();
            let identifier = match expr_any.downcast::<Identifier>() {
                Ok(v) => v,
                Err(e) => panic!("expected an identifier statement, found {:?}", e),
            };

            assert_eq!(identifier.value, "foobar");
            assert_eq!(identifier.token_literal(), "foobar");
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
}
