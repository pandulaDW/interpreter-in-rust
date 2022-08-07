#![allow(dead_code)]

use crate::ast::program::Program;
use crate::lexer::token::{eof_token, Token};
use crate::lexer::Lexer;

struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
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
        };

        // Read two tokens, so curToken and peekToken are both set
        p = p.next_token();
        p = p.next_token();

        p
    }

    // Sets the peek token to the current token and advances to a new token.
    fn next_token(mut self) -> Self {
        self.current_token = self.peek_token;
        self.peek_token = self.l.next_token();
        self
    }

    pub fn parse_program(&self) -> Program {
        let program = Program::new();
        program
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Parser};
    use crate::ast::statements::LetStatement;
    use crate::ast::Node;

    #[test]
    fn test_let_statements() {
        let input = " let x = 5;
        let y = 10;
        let foobar = 838383;";

        let l = Lexer::new(input);
        let p = Parser::new(l);

        let program = p.parse_program();
        assert!(
            program.statements.len() == 3,
            "program.Statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let test_cases = vec!["x", "y", "foobar"];

        for (i, stmt) in program.statements.into_iter().enumerate() {
            let stmt_any = stmt.into_any();
            let expected_name = test_cases[i];

            let let_stmt = match stmt_any.downcast_ref::<LetStatement>() {
                Some(val) => val,
                None => panic!("expected a let statement, found something else"),
            };

            test_let_statement(let_stmt, expected_name);
        }
    }

    fn test_let_statement(s: &LetStatement, name: &str) {
        assert_eq!(
            s.token_literal(),
            "let",
            "token literal is not let, got {}",
            s.token_literal()
        );

        assert_eq!(
            s.name.value, name,
            "name is not {}. got {}",
            s.name.value, name
        );

        assert_eq!(
            s.name.token_literal(),
            name,
            "name is not {}. got {}",
            s.name.value,
            name
        );
    }
}
