mod tree;

/// Every node in the AST has to implement the Node interface, meaning it has
/// to provide a TokenLiteral() method that returns the literal value of
/// the token it’s associated with.
pub trait Node {
    fn token_literal(&self) -> String;
}

/// Should be implemented by statements as a way of differentiating between expressions
trait Statement: Node {
    fn statement_node();
}

/// Should be implemented by expressions as a way of differentiating between statements
trait Expression: Node {
    fn expression_node();
}
