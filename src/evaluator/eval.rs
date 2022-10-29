use crate::{
    ast::{
        expressions::{AllExpressions, IfExpression},
        statements::{AllStatements, BlockStatement},
        AllNodes,
    },
    object::{
        objects::{Boolean, Integer, Null},
        AllObjects,
    },
};

// constants that can be reused without extra allocations
const TRUE: AllObjects = AllObjects::Boolean(Boolean { value: true });
const FALSE: AllObjects = AllObjects::Boolean(Boolean { value: false });
const NULL: AllObjects = AllObjects::Null(Null);

/// eval takes in any type of node and applies the appropriate evaluation logic
pub fn eval(node: AllNodes) -> Option<AllObjects> {
    match node {
        AllNodes::Program(p) => eval_program(p.statements),
        AllNodes::Statements(s) => eval_statement(s),
        AllNodes::Expressions(e) => eval_expression(e),
    }
}

fn eval_program(stmts: Vec<AllStatements>) -> Option<AllObjects> {
    let mut result = None;

    for stmt in stmts {
        result = eval(AllNodes::Statements(stmt));

        // if the value is a ReturnValue, return early
        match result {
            Some(v) => {
                if let AllObjects::ReturnValue(v) = v {
                    return Some(*v);
                } else {
                    result = Some(v)
                }
            }
            None => {
                result = None;
            }
        }
    }

    result
}

fn eval_statement(stmt: AllStatements) -> Option<AllObjects> {
    match stmt {
        AllStatements::Let(_) => None,
        AllStatements::Return(stmt) => {
            let evaluated = eval(AllNodes::Expressions(*stmt.return_value))?;
            Some(AllObjects::ReturnValue(Box::new(evaluated)))
        }
        AllStatements::Expression(stmt) => eval_expression(*stmt.expression?),
        AllStatements::_Block(block) => eval_block_statement(block),
    }
}

fn eval_block_statement(block: BlockStatement) -> Option<AllObjects> {
    let mut result = None;

    for stmt in block.statements {
        result = eval(AllNodes::Statements(stmt));

        match result {
            Some(ref v) => {
                if let AllObjects::ReturnValue(_) = v {
                    return result;
                }
            }
            None => {
                result = None;
            }
        }
    }

    result
}

fn eval_expression(exprs: AllExpressions) -> Option<AllObjects> {
    match exprs {
        AllExpressions::IntegerLiteral(node) => {
            Some(AllObjects::Integer(Integer { value: node.value }))
        }

        AllExpressions::Boolean(node) => {
            if node.value {
                return Some(TRUE);
            }
            Some(FALSE)
        }

        AllExpressions::PrefixExpression(node) => {
            let right = node.right?;
            let right_evaluated = eval(AllNodes::Expressions(*right))?;
            Some(eval_prefix_expression(&node.operator, right_evaluated))
        }

        AllExpressions::InfixExpression(node) => {
            let left = node.left?;
            let right = node.right?;

            let left_eval = eval(AllNodes::Expressions(*left))?;
            let right_eval = eval(AllNodes::Expressions(*right))?;

            Some(eval_infix_expression(left_eval, &node.operator, right_eval))
        }

        AllExpressions::IfExpression(node) => eval_if_expression(node),

        _ => None,
    }
}

fn eval_prefix_expression(operator: &str, right: AllObjects) -> AllObjects {
    match operator {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_operator(right),
        _ => NULL,
    }
}

fn eval_infix_expression(left: AllObjects, operator: &str, right: AllObjects) -> AllObjects {
    if left.is_integer() && right.is_integer() {
        return eval_integer_calculations(left, operator, right);
    }
    if left.is_boolean() && right.is_boolean() {
        return eval_comparison_for_booleans(left, operator, right);
    }
    NULL
}

fn eval_if_expression(expr: IfExpression) -> Option<AllObjects> {
    let condition = eval(AllNodes::Expressions(*expr.condition))?;

    if is_truthy(condition) {
        return eval_block_statement(expr.consequence);
    }

    if expr.alternative.is_none() {
        return Some(NULL);
    }

    let alternative = expr.alternative?;
    eval_block_statement(alternative)
}

fn eval_bang_operator(right: AllObjects) -> AllObjects {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_operator(right: AllObjects) -> AllObjects {
    if let AllObjects::Integer(v) = right {
        return AllObjects::Integer(Integer { value: -v.value });
    }

    NULL
}

fn eval_integer_calculations(left: AllObjects, operator: &str, right: AllObjects) -> AllObjects {
    let left_int_val = match left {
        AllObjects::Integer(v) => v,
        _ => return NULL,
    }
    .value;

    let right_int_val = match right {
        AllObjects::Integer(v) => v,
        _ => return NULL,
    }
    .value;

    match operator {
        "+" => AllObjects::Integer(Integer {
            value: left_int_val + right_int_val,
        }),
        "-" => AllObjects::Integer(Integer {
            value: left_int_val - right_int_val,
        }),
        "*" => AllObjects::Integer(Integer {
            value: left_int_val * right_int_val,
        }),
        "/" => AllObjects::Integer(Integer {
            value: left_int_val / right_int_val,
        }),
        "<" => get_bool_consts(left_int_val < right_int_val),
        ">" => get_bool_consts(left_int_val > right_int_val),
        "!=" => get_bool_consts(left_int_val != right_int_val),
        "==" => get_bool_consts(left_int_val == right_int_val),
        _ => NULL,
    }
}

fn eval_comparison_for_booleans(left: AllObjects, operator: &str, right: AllObjects) -> AllObjects {
    let left_val = match left {
        AllObjects::Boolean(v) => v,
        _ => return NULL,
    };

    let right_val = match right {
        AllObjects::Boolean(v) => v,
        _ => return NULL,
    };

    match operator {
        "==" => get_bool_consts(left_val.value == right_val.value),
        "!=" => get_bool_consts(left_val.value != right_val.value),
        _ => NULL,
    }
}

fn is_truthy(obj: AllObjects) -> bool {
    match obj {
        AllObjects::Boolean(v) => v.value,
        AllObjects::Null(_) => false,
        _ => true,
    }
}

fn get_bool_consts(val: bool) -> AllObjects {
    if val {
        return TRUE;
    }
    FALSE
}
