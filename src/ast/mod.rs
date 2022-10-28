pub mod expressions;
pub mod program;
pub mod statements;

use std::{any::Any, fmt::Display};

/// Every node in the AST has to implement the Node interface, meaning it has
/// to provide a TokenLiteral() method that returns the literal value of
/// the token it’s associated with.
pub trait Node: Display {
    /// Return the token literal as a String
    fn token_literal(&self) -> String;

    /// Converts a boxed `Statement` trait object into a boxed Any trait object.
    ///
    /// This is required for runtime type down-casting.
    /// Since the program keeps a list of `Statement`s and we would want to infer the underlying
    /// type that implements the `Statement` trait.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Should be implemented by statements as a way of differentiating between expressions
pub trait Statement: Node {}

/// Should be implemented by expressions as a way of differentiating between statements
pub trait Expression: Node {}

#[cfg(test)]
mod tests {
    use super::{
        expressions::Identifier,
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
            value: Some(Box::new(value)),
        };

        program.statements.push(AllStatements::Let(stmt));
        assert_eq!(program.to_string(), "let myVar = anotherVar;\n");
    }
}
