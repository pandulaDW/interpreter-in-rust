use crate::{
    ast::expressions::{FunctionLiteral, IntegerLiteral, StringLiteral},
    object::{
        objects::{Boolean, Function, Integer, Null, StringObj},
        AllObjects,
    },
    Environment,
};

use std::rc::Rc;
use uuid::Uuid;

// constants that can be reused without extra allocations
const TRUE: AllObjects = AllObjects::Boolean(Boolean { value: true });
const FALSE: AllObjects = AllObjects::Boolean(Boolean { value: false });
pub const NULL: AllObjects = AllObjects::Null(Null);

pub fn new_function_literal(node: FunctionLiteral, env: Rc<Environment>) -> AllObjects {
    let name = format!("fn_{}", Uuid::new_v4());

    AllObjects::Function(Function {
        name,
        body: node.body,
        env,
        parameters: node.parameters,
    })
}

pub fn is_truthy(obj: &AllObjects) -> bool {
    match obj {
        AllObjects::Boolean(v) => v.value,
        AllObjects::Null(_) => false,
        _ => true,
    }
}

pub fn get_bool_consts(val: bool) -> AllObjects {
    if val {
        return TRUE;
    }
    FALSE
}

pub fn get_int_object(node: IntegerLiteral) -> AllObjects {
    AllObjects::Integer(Integer { value: node.value })
}

pub fn get_int_object_for_value(value: i64) -> AllObjects {
    AllObjects::Integer(Integer { value })
}

pub fn get_string_object(node: StringLiteral) -> AllObjects {
    AllObjects::StringObj(StringObj {
        value: Rc::new(node.token.literal),
    })
}

pub fn eval_bang_operator(right: AllObjects) -> AllObjects {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}
