use super::builtins;
use super::errors;
use super::helpers::{self, *};

use crate::object::objects::BuiltinFunctionObj;
use crate::object::objects::FunctionObj;
use crate::object::objects::HashMapObj;
use crate::{
    ast::{expressions::*, statements::*, AllNodes},
    object::{
        environment::Environment,
        objects::{ArrayObj, Boolean, Integer, ParamsType, StringObj},
        AllObjects,
    },
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// eval takes in any type of node and applies the appropriate evaluation logic
pub fn eval(node: AllNodes, env: Rc<Environment>) -> Option<AllObjects> {
    match node {
        AllNodes::Program(p) => eval_program(p.statements, env),
        AllNodes::Statements(s) => eval_statement(s, env),
        AllNodes::Expressions(e) => eval_expression(e, env),
    }
}

fn eval_program(stmts: Vec<AllStatements>, env: Rc<Environment>) -> Option<AllObjects> {
    let mut result = None;

    for stmt in stmts {
        result = eval(AllNodes::Statements(stmt), env.clone());

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

fn eval_statement(stmt: AllStatements, env: Rc<Environment>) -> Option<AllObjects> {
    match stmt {
        AllStatements::Let(stmt) => eval_let_statement(stmt, env),
        AllStatements::Return(stmt) => eval_return_statement(stmt, env),
        AllStatements::Expression(stmt) => eval_expression(*stmt.expression?, env),
        AllStatements::Block(block) => eval_block_statement(block, env),
        AllStatements::While(stmt) => eval_while_statement(stmt, env),
    }
}

fn eval_let_statement(stmt: LetStatement, env: Rc<Environment>) -> Option<AllObjects> {
    let value = eval(AllNodes::Expressions(*stmt.value), env.clone())?;
    if value.is_error() {
        return Some(value);
    }
    Some(env.set(stmt.name.value, value))
}

fn eval_block_statement(block: BlockStatement, env: Rc<Environment>) -> Option<AllObjects> {
    let mut result = None;

    for stmt in block.statements {
        result = eval(AllNodes::Statements(stmt), env.clone());

        if let Some(ref v) = result {
            match v {
                AllObjects::ReturnValue(_) | AllObjects::Error(_) => return result,
                _ => {}
            }
        }
    }

    result
}

fn eval_return_statement(stmt: ReturnStatement, env: Rc<Environment>) -> Option<AllObjects> {
    let evaluated = eval(AllNodes::Expressions(*stmt.return_value), env)?;
    if evaluated.is_error() {
        return Some(evaluated);
    }
    Some(AllObjects::ReturnValue(Box::new(evaluated)))
}

fn eval_while_statement(stmt: WhileStatement, env: Rc<Environment>) -> Option<AllObjects> {
    let mut condition = eval(AllNodes::Expressions(*stmt.condition.clone()), env.clone())?;
    if condition.is_error() {
        return Some(condition);
    }

    let new_env = Environment::new_enclosed_environment(env.clone());

    while is_truthy(&condition) {
        let result = eval_block_statement(stmt.body.clone(), new_env.clone())?;

        match result {
            AllObjects::ReturnValue(_) | AllObjects::Error(_) => return Some(result),
            _ => {}
        }

        condition = eval(AllNodes::Expressions(*stmt.condition.clone()), env.clone())?;
        if condition.is_error() {
            return Some(condition);
        }
    }

    Some(helpers::NULL)
}

fn eval_expression(exprs: AllExpressions, env: Rc<Environment>) -> Option<AllObjects> {
    match exprs {
        AllExpressions::IntegerLiteral(node) => Some(get_int_object(node)),
        AllExpressions::StringLiteral(node) => Some(get_string_object(node)),
        AllExpressions::Boolean(node) => Some(get_bool_consts(node.value)),
        AllExpressions::Assignment(node) => eval_assignment_expression(node, env),
        AllExpressions::PrefixExpression(node) => eval_prefix_expression(node, env),
        AllExpressions::InfixExpression(node) => eval_infix_expression(node, env),
        AllExpressions::IfExpression(node) => eval_if_expression(node, env),
        AllExpressions::Identifier(node) => eval_identifier(node, env),
        AllExpressions::FunctionLiteral(node) => Some(new_function_literal(node, env)),
        AllExpressions::CallExpression(node) => eval_call_expression(node, env),
        AllExpressions::ArrayLiteral(node) => eval_array_literal(node, env),
        AllExpressions::NullLiteral => Some(NULL),
        AllExpressions::IndexExpression(node) => eval_index_expression(node, env),
        AllExpressions::RangeExpression(node) => eval_range_expression(node, env),
        AllExpressions::HashLiteral(node) => eval_hash_map(node, env),
    }
}

fn eval_assignment_expression(
    node: AssignmentExpression,
    env: Rc<Environment>,
) -> Option<AllObjects> {
    let ident = node.ident;
    let evaluated = eval(AllNodes::Expressions(*node.value), env.clone())?;

    match env.replace(&ident.value, evaluated) {
        Some(v) => Some(v),
        None => Some(errors::identifier_not_found(&ident.value)),
    }
}

fn eval_prefix_expression(node: PrefixExpression, env: Rc<Environment>) -> Option<AllObjects> {
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

fn eval_infix_expression(node: InfixExpression, env: Rc<Environment>) -> Option<AllObjects> {
    let left = eval(AllNodes::Expressions(*node.left?), env.clone())?;
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
    if left.is_string() && right.is_string() {
        return Some(eval_string_operations(left, &node.operator, right));
    }

    Some(errors::unknown_operator(
        Some(&left),
        &node.operator,
        &right,
    ))
}

fn eval_if_expression(expr: IfExpression, env: Rc<Environment>) -> Option<AllObjects> {
    let condition = eval(AllNodes::Expressions(*expr.condition), env.clone())?;
    if condition.is_error() {
        return Some(condition);
    }

    let new_env = Environment::new_enclosed_environment(env);
    if is_truthy(&condition) {
        return eval_block_statement(expr.consequence, new_env);
    }

    if expr.alternative.is_none() {
        return Some(NULL);
    }

    let alternative = expr.alternative?;
    eval_block_statement(alternative, new_env)
}

/// Returns the associated object from the environment.
///
/// If the value is not found, an additional check is performed on the builtins.
fn eval_identifier(node: Identifier, env: Rc<Environment>) -> Option<AllObjects> {
    let ident = env.get(&node.value);
    if ident.is_none() {
        let builtin_function = builtins::get_builtin_function(&node);
        if builtin_function.is_some() {
            return builtin_function;
        }
        return Some(errors::identifier_not_found(&node.value));
    }
    ident
}

fn eval_call_expression(node: CallExpression, env: Rc<Environment>) -> Option<AllObjects> {
    let function = eval(AllNodes::Expressions(*node.function), env.clone())?;
    if function.is_error() {
        return Some(function);
    }

    let mut args = eval_expressions(node.arguments, env)?;
    if args.len() == 1 && args[0].is_error() {
        return Some(args.remove(0));
    }

    if let AllObjects::Function(f) = function {
        return eval_user_defined_function_call(f, args);
    }

    if let AllObjects::BuiltinFunction(f) = function {
        return eval_builtin_function_calls(f, args);
    }

    None
}

fn eval_user_defined_function_call(f: FunctionObj, args: Vec<AllObjects>) -> Option<AllObjects> {
    let func_env = Environment::new_enclosed_environment(f.env);

    if f.parameters.len() != args.len() {
        return Some(errors::incorrect_arg_num(f.parameters.len(), args.len()));
    }

    for (param_idx, param) in f.parameters.iter().enumerate() {
        func_env.set(param.value.clone(), args[param_idx].clone());
    }

    let evaluated = eval_block_statement(f.body, func_env);

    if let Some(AllObjects::ReturnValue(r_val)) = evaluated {
        return Some(*r_val);
    }
    return evaluated;
}

fn eval_builtin_function_calls(f: BuiltinFunctionObj, args: Vec<AllObjects>) -> Option<AllObjects> {
    let new_env = Environment::new();

    match f.parameters {
        ParamsType::Fixed(v) => {
            if v.len() != args.len() {
                return Some(errors::incorrect_arg_num(v.len(), args.len()));
            }
            v.iter().enumerate().for_each(|(param_idx, param)| {
                new_env.set(param.clone(), args[param_idx].clone());
            })
        }
        ParamsType::Variadic => args.into_iter().enumerate().for_each(|(i, arg)| {
            new_env.set(format!("arg_{}", i), arg);
        }),
    }

    return Some((f.func)(new_env));
}

fn eval_array_literal(node: ArrayLiteral, env: Rc<Environment>) -> Option<AllObjects> {
    let mut v = Vec::with_capacity(node.elements.len());
    for expr in node.elements {
        let evaluated = eval(AllNodes::Expressions(expr), env.clone())?;
        v.push(evaluated);
    }

    Some(AllObjects::ArrayObj(ArrayObj {
        elements: Rc::new(RefCell::new(v)),
    }))
}

fn eval_index_expression(node: IndexExpression, env: Rc<Environment>) -> Option<AllObjects> {
    let evaluated_left = eval(AllNodes::Expressions(*node.left), env.clone())?;
    let evaluated_index = eval(AllNodes::Expressions(*node.index), env)?;

    if let AllObjects::HashMap(v) = &evaluated_left {
        return Some(get_hash_map_value(v, &evaluated_index));
    }

    let index = match evaluated_index {
        AllObjects::Integer(v) => v,
        other => return Some(errors::unexpected_argument_type("an INTEGER", other)),
    };

    let index: usize = match index.value.try_into() {
        Ok(v) => v,
        Err(_) => return Some(errors::incorrect_index_argument()),
    };

    let val = match evaluated_left {
        AllObjects::ArrayObj(v) => get_array_index_value(v, index, None),
        AllObjects::StringObj(v) => get_string_index_value(v, index, None),
        other => {
            return Some(errors::unexpected_argument_type(
                "an ARRAY or a STRING",
                other,
            ))
        }
    };

    Some(val)
}

/// Evaluate range expressions and returns a clone of the indexed slice of an array
fn eval_range_expression(node: RangeExpression, env: Rc<Environment>) -> Option<AllObjects> {
    let left = match eval(AllNodes::Expressions(*node.left_index), env.clone())? {
        AllObjects::Integer(v) => v,
        other => return Some(errors::unexpected_argument_type("an INTEGER", other)),
    };

    let right = match eval(AllNodes::Expressions(*node.right_index), env.clone())? {
        AllObjects::Integer(v) => v,
        other => return Some(errors::unexpected_argument_type("an INTEGER", other)),
    };

    let Ok(left_index) = TryInto::<usize>::try_into(left.value) else {
        return Some(errors::incorrect_index_argument());
    };

    let Ok(right_index) = TryInto::<usize>::try_into(right.value) else {
        return Some(errors::incorrect_index_argument());
    };

    let val = match eval(AllNodes::Expressions(*node.left), env)? {
        AllObjects::ArrayObj(v) => get_array_index_value(v, left_index, Some(right_index)),
        AllObjects::StringObj(v) => get_string_index_value(v, left_index, Some(right_index)),
        other => {
            return Some(errors::unexpected_argument_type(
                "an ARRAY or a STRING",
                other,
            ))
        }
    };

    Some(val)
}

fn eval_hash_map(node: HashLiteral, env: Rc<Environment>) -> Option<AllObjects> {
    let mut map = HashMap::new();

    for pair in node.pairs {
        let key = eval(AllNodes::Expressions(pair.0), env.clone())?;
        let value = eval(AllNodes::Expressions(pair.1), env.clone())?;
        map.insert(key, value);
    }

    Some(AllObjects::HashMap(HashMapObj {
        map: Rc::new(RefCell::new(map)),
    }))
}

fn eval_expressions(exprs: Vec<AllExpressions>, env: Rc<Environment>) -> Option<Vec<AllObjects>> {
    let mut v = Vec::with_capacity(exprs.len());

    for expr in exprs {
        let evaluated = eval(AllNodes::Expressions(expr), env.clone())?;
        if evaluated.is_error() {
            return Some(vec![evaluated]);
        }
        v.push(evaluated);
    }

    Some(v)
}

fn eval_minus_operator(right: AllObjects) -> AllObjects {
    if let AllObjects::Integer(v) = right {
        return AllObjects::Integer(Integer { value: -v.value });
    }
    errors::unknown_operator(None, "-", &right)
}

fn eval_integer_calculations(left: AllObjects, operator: &str, right: AllObjects) -> AllObjects {
    let left_int = match left {
        AllObjects::Integer(v) => v,
        _ => return NULL,
    }
    .value;

    let right_int = match right {
        AllObjects::Integer(v) => v,
        _ => return NULL,
    }
    .value;

    match operator {
        "+" => get_int_object_for_value(left_int + right_int),
        "-" => get_int_object_for_value(left_int - right_int),
        "*" => get_int_object_for_value(left_int * right_int),
        "/" => get_int_object_for_value(left_int / right_int),
        "<" => get_bool_consts(left_int < right_int),
        ">" => get_bool_consts(left_int > right_int),
        "!=" => get_bool_consts(left_int != right_int),
        "==" => get_bool_consts(left_int == right_int),
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

fn eval_string_operations(left: AllObjects, operator: &str, right: AllObjects) -> AllObjects {
    let left_val = match &left {
        AllObjects::StringObj(v) => v,
        _ => return NULL,
    }
    .value
    .clone();

    let right_val = match &right {
        AllObjects::StringObj(v) => v,
        _ => return NULL,
    }
    .value
    .clone();

    match operator {
        "+" => AllObjects::StringObj(StringObj {
            value: Rc::new(format!("{}{}", left_val, right_val)),
        }),
        ">" | "==" | "<" | "!=" => {
            let Some(v) = eval_string_comparisons(left_val, operator, right_val) else {
              return errors::unknown_operator(Some(&left), operator, &right);
            };
            v
        }
        _ => errors::unknown_operator(Some(&left), operator, &right),
    }
}

fn eval_string_comparisons(
    left: Rc<String>,
    operator: &str,
    right: Rc<String>,
) -> Option<AllObjects> {
    let value = match operator {
        ">" => left > right,
        "<" => left < right,
        "==" => left == right,
        "!=" => left != right,
        _ => return None,
    };
    Some(AllObjects::Boolean(Boolean { value }))
}
