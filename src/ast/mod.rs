use std::any::Any;

pub mod program;
pub mod statements;

/// Every node in the AST has to implement the Node interface, meaning it has
/// to provide a TokenLiteral() method that returns the literal value of
/// the token it’s associated with.
pub trait Node {
    fn token_literal(&self) -> String;
}

/// Should be implemented by statements as a way of differentiating between expressions
pub trait Statement: Node {
    /// A marker method to mark a statement
    fn statement_node(&self) {}

    /// Converts a boxed `Statement` trait object into a boxed Any trait object.
    ///
    /// This is required for runtime type down-casting.
    /// Since the program keeps a list of `Statement`s and we would want to infer the underlying
    /// type that implements the `Statement` trait.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Should be implemented by expressions as a way of differentiating between statements
pub trait Expression: Node {
    fn expression_node(&self) {}
}
