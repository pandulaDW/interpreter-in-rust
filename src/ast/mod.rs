pub mod expressions;
pub mod program;
pub mod statements;

use self::{expressions::AllExpressions, program::Program, statements::AllStatements};

/// A Wrapper around the program node, statement nodes and expression nodes
///
/// Primarily will be used by the evaluator.
pub enum AllNodes {
    Program(Program),
    Statements(AllStatements),
    Expressions(AllExpressions),
}

#[cfg(test)]
mod tests {
    use super::{
        expressions::{AllExpressions, Identifier},
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
            value: Box::new(AllExpressions::Identifier(value)),
        };

        program.statements.push(AllStatements::Let(stmt));
        assert_eq!(program.to_string(), "let myVar = anotherVar;\n");
    }
}
