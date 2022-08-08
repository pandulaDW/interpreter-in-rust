#![allow(dead_code)]

use std::mem;

use crate::ast::program::Program;
use crate::lexer::token::{eof_token, Token, TokenType};
use crate::lexer::Lexer;

pub struct Parser {
    l: Lexer,
    pub current_token: Token,
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
        p.next_token();
        p.next_token();

        p
    }

    /// Sets the peek token to the current token and advances to a new token.
    pub fn next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.l.next_token();
    }

    /// Checks if the current token is the supplied token type
    pub fn current_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    /// Checks if the peek token is the supplied token type
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    /// If the peek token is the supplied token type, advances the lexer
    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        false
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
