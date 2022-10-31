use super::errors::{self, identifier_not_found};
use crate::{
    ast::{
        expressions::{
            AllExpressions, Identifier, IfExpression, InfixExpression, IntegerLiteral,
            PrefixExpression,
        },
        statements::{AllStatements, BlockStatement, LetStatement, ReturnStatement},
        AllNodes,
    },
    object::{
        objects::{Boolean, Environment, Integer, Null},
        AllObjects,
    },
};

// constants that can be reused without extra allocations
const TRUE: AllObjects = AllObjects::Boolean(Boolean { value: true });
const FALSE: AllObjects = AllObjects::Boolean(Boolean { value: false });
const NULL: AllObjects = AllObjects::Null(Null);

/// eval takes in any type of node and applies the appropriate evaluation logic
pub fn eval(node: AllNodes, env: &mut Environment) -> Option<AllObjects> {
    match node {
        AllNodes::Program(p) => eval_program(p.statements, env),
        AllNodes::Statements(s) => eval_statement(s, env),
        AllNodes::Expressions(e) => eval_expression(e, env),
    }
}

fn eval_program(stmts: Vec<AllStatements>, env: &mut Environment) -> Option<AllObjects> {
    let mut result = None;

    for stmt in stmts {
        result = eval(AllNodes::Statements(stmt), env);

        // if the value is a ReturnValue then return early with its underlying value.
        // if the value is an error, return early with the error
        match result {
            Some(v) => match v {
                AllObjects::ReturnValue(r_val) => return Some(*r_val),
                AllObjects::Error(_) => return Some(v),
                _ => result = Some(v),
            },
            None => {
                result = None;
            }
        }
    }

    result
}

fn eval_statement(stmt: AllStatements, env: &mut Environment) -> Option<AllObjects> {
    match stmt {
        AllStatements::Let(stmt) => eval_let_statement(stmt, env),
        AllStatements::Return(stmt) => eval_return_statement(stmt, env),
        AllStatements::Expression(stmt) => eval_expression(*stmt.expression?, env),
        AllStatements::_Block(block) => eval_block_statement(block, env),
    }
}

fn eval_let_statement(stmt: LetStatement, env: &mut Environment) -> Option<AllObjects> {
    let value = eval(AllNodes::Expressions(*stmt.value), env)?;
    if value.is_error() {
        return Some(value);
    }

    Some(env.set(stmt.name.value, value))
}

fn eval_block_statement(block: BlockStatement, env: &mut Environment) -> Option<AllObjects> {
    let mut result = None;

    for stmt in block.statements {
        result = eval(AllNodes::Statements(stmt), env);

        if let Some(ref v) = result {
            match v {
                AllObjects::ReturnValue(_) | AllObjects::Error(_) => return result,
                _ => {}
            }
        }
    }

    result
}

fn eval_return_statement(stmt: ReturnStatement, env: &mut Environment) -> Option<AllObjects> {
    let evaluated = eval(AllNodes::Expressions(*stmt.return_value), env)?;
    if evaluated.is_error() {
        return Some(evaluated);
    }
    Some(AllObjects::ReturnValue(Box::new(evaluated)))
}

fn eval_expression(exprs: AllExpressions, env: &mut Environment) -> Option<AllObjects> {
    match exprs {
        AllExpressions::IntegerLiteral(node) => Some(get_int_object(node)),
        AllExpressions::Boolean(node) => Some(get_bool_consts(node.value)),
        AllExpressions::PrefixExpression(node) => eval_prefix_expression(node, env),
        AllExpressions::InfixExpression(node) => eval_infix_expression(node, env),
        AllExpressions::IfExpression(node) => eval_if_expression(node, env),
        AllExpressions::Identifier(node) => eval_identifier(node, env),
        AllExpressions::FunctionLiteral(node) => Some(AllObjects::new_function(node)),
        _ => None,
    }
}

fn eval_prefix_expression(node: PrefixExpression, env: &mut Environment) -> Option<AllObjects> {
    let right = node.right?;
    let right_evaluated = eval(AllNodes::Expressions(*right), env)?;

    if right_evaluated.is_error() {
        return Some(right_evaluated);
    }

    let result = match node.operator.as_str() {
        "!" => eval_bang_operator(right_evaluated),
        "-" => eval_minus_operator(right_evaluated),
        _ => NULL,
    };

    Some(result)
}

fn eval_infix_expression(node: InfixExpression, env: &mut Environment) -> Option<AllObjects> {
    let left = eval(AllNodes::Expressions(*node.left?), env)?;
    if left.is_error() {
        return Some(left);
    }

    let right = eval(AllNodes::Expressions(*node.right?), env)?;
    if right.is_error() {
        return Some(right);
    }

    if left.object_type() != right.object_type() {
        return Some(errors::type_mismatch(&left, &node.operator, &right));
    };

    if left.is_integer() && right.is_integer() {
        return Some(eval_integer_calculations(left, &node.operator, right));
    }
    if left.is_boolean() && right.is_boolean() {
        return Some(eval_comparison_for_booleans(left, &node.operator, right));
    }

    Some(errors::unknown_operator(
        Some(&left),
        &node.operator,
        &right,
    ))
}

fn eval_if_expression(expr: IfExpression, env: &mut Environment) -> Option<AllObjects> {
    let condition = eval(AllNodes::Expressions(*expr.condition), env)?;
    if condition.is_error() {
        return Some(condition);
    }

    if is_truthy(condition) {
        return eval_block_statement(expr.consequence, env);
    }

    if expr.alternative.is_none() {
        return Some(NULL);
    }

    let alternative = expr.alternative?;
    eval_block_statement(alternative, env)
}

fn eval_identifier(node: Identifier, env: &mut Environment) -> Option<AllObjects> {
    let ident = env.get(&node.value);
    if ident.is_none() {
        return Some(identifier_not_found(&node.value));
    }
    ident.cloned()
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
    errors::unknown_operator(None, "-", &right)
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
    let left_val = match &left {
        AllObjects::Boolean(v) => v,
        _ => return NULL,
    };

    let right_val = match &right {
        AllObjects::Boolean(v) => v,
        _ => return NULL,
    };

    match operator {
        "==" => get_bool_consts(left_val.value == right_val.value),
        "!=" => get_bool_consts(left_val.value != right_val.value),
        _ => errors::unknown_operator(Some(&left), operator, &right),
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

fn get_int_object(node: IntegerLiteral) -> AllObjects {
    AllObjects::Integer(Integer { value: node.value })
}
