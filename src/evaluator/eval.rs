use crate::{
    ast::{expressions::AllExpressions, statements::AllStatements, AllNodes},
    object::{objects::Integer, AllObjects},
};

/// eval takes in any type of node and applies the appropriate logic
pub fn eval(node: AllNodes) -> Option<AllObjects> {
    match node {
        AllNodes::Program(p) => eval_statements(p.statements),
        AllNodes::Statements(s) => eval_statement(s),
    }
}

fn eval_statements(stmts: Vec<AllStatements>) -> Option<AllObjects> {
    let mut result = None;

    for stmt in stmts {
        result = eval(AllNodes::Statements(stmt));
    }

    result
}

fn eval_statement(stmt: AllStatements) -> Option<AllObjects> {
    match stmt {
        AllStatements::Let(_) => todo!(),
        AllStatements::Return(_) => todo!(),
        AllStatements::Expression(expr_stmt) => eval_expression(*expr_stmt.expression?),
    }
}

fn eval_expression(exprs: AllExpressions) -> Option<AllObjects> {
    match exprs {
        AllExpressions::IntegerLiteral(node) => {
            Some(AllObjects::Integer(Integer { value: node.value }))
        }
        _ => None,
    }
}
