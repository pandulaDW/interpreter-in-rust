use crate::{
    ast::{expressions::AllExpressions, statements::AllStatements, AllNodes},
    object::{
        objects::{Boolean, Integer, Null},
        AllObjects,
    },
};

const TRUE: AllObjects = AllObjects::Boolean(Boolean { value: true });
const FALSE: AllObjects = AllObjects::Boolean(Boolean { value: false });
const NULL: AllObjects = AllObjects::Null(Null);

/// eval takes in any type of node and applies the appropriate logic
pub fn eval(node: AllNodes) -> Option<AllObjects> {
    match node {
        AllNodes::Program(p) => eval_statements(p.statements),
        AllNodes::Statements(s) => eval_statement(s),
        AllNodes::Expressions(e) => eval_expression(e),
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

        AllExpressions::Boolean(node) => {
            if node.value {
                Some(TRUE)
            } else {
                Some(FALSE)
            }
        }

        AllExpressions::PrefixExpression(node) => {
            let right = node.right?;
            let right_evaluated = eval(AllNodes::Expressions(*right))?;
            Some(eval_prefix_expression(node.operator, right_evaluated))
        }

        _ => None,
    }
}

fn eval_prefix_expression(operator: String, right: AllObjects) -> AllObjects {
    match operator.as_str() {
        "!" => eval_bang_op_expression(right),
        _ => NULL,
    }
}

fn eval_bang_op_expression(right: AllObjects) -> AllObjects {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}
