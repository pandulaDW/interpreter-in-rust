pub mod expressions;
pub mod program;
pub mod statements;

use std::fmt::Display;

/// Every node in the AST has to implement the Node interface, meaning it has
/// to provide a TokenLiteral() method that returns the literal value of
/// the token it’s associated with.
pub trait Node: Display {
    /// Return the token literal as a String
    fn token_literal(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::{
        expressions::{AllExpression, Identifier},
        program::Program,
        statements::{AllStatements, LetStatement},
    };
    use crate::lexer::{
        keywords,
        token::{new_token, TokenType},
    };

    #[test]
    fn test_string() {
        let mut program = Program::new();

        let name = Identifier {
            token: new_token(TokenType::Ident, "myVar"),
            value: "myVar".to_string(),
        };

        let value = Identifier {
            token: new_token(TokenType::Ident, "anotherVar"),
            value: "anotherVar".to_string(),
        };

        let stmt = LetStatement {
            token: new_token(TokenType::Let, keywords::LET),
            name,
            value: Box::new(AllExpression::Identifier(value)),
        };

        program.statements.push(AllStatements::Let(stmt));
        assert_eq!(program.to_string(), "let myVar = anotherVar;\n");
    }
}
