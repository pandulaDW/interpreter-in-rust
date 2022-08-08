#![allow(dead_code)]

use crate::ast::program::Program;
use crate::lexer::token::{eof_token, Token, TokenType};
use crate::lexer::Lexer;

pub struct Parser {
    pub l: Lexer,
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
        };

        // Read two tokens, so curToken and peekToken are both set
        p.next_token();
        p.next_token();

        p
    }

    /// The main parser method, which iterates through the tokens and generates a list of AST statements
    /// within the `Program`
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
    use crate::ast::statements::LetStatement;
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
        assert!(program.statements.len() == 3);

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
        assert_eq!(s.token_literal(), keywords::LET);
        assert_eq!(s.name.value, name);
        assert_eq!(s.name.token_literal(), name);
    }
}
