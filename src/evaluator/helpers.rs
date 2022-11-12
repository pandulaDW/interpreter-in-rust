use crate::{
    ast::expressions::{FunctionLiteral, IntegerLiteral, StringLiteral},
    object::{
        objects::{ArrayObj, Boolean, FunctionObj, HashMapObj, Integer, Null, StringObj},
        AllObjects,
    },
    Environment,
};

use super::errors;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

// constants that can be reused without extra allocations
const TRUE: AllObjects = AllObjects::Boolean(Boolean { value: true });
const FALSE: AllObjects = AllObjects::Boolean(Boolean { value: false });
pub const NULL: AllObjects = AllObjects::Null(Null);

pub fn new_function_literal(node: FunctionLiteral, env: Rc<Environment>) -> AllObjects {
    let name = format!("fn_{}", Uuid::new_v4());

    AllObjects::Function(FunctionObj {
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

pub fn get_array_index_value(
    array: ArrayObj,
    left_index: usize,
    right_index: Option<usize>,
) -> AllObjects {
    let binding = array.elements.borrow();

    if let Some(right) = right_index {
        let Some(slice) = binding.get(left_index..right) else {
            return errors::indexing_error();
        };
        let cloned_slice = Rc::new(RefCell::new(slice.to_vec()));
        return AllObjects::ArrayObj(ArrayObj {
            elements: cloned_slice,
        });
    }

    let Some(val) = binding.get(left_index) else {
        return errors::indexing_error();
    };

    val.clone()
}

pub fn get_string_index_value(
    str: StringObj,
    left_index: usize,
    right_index: Option<usize>,
) -> AllObjects {
    if let Some(right) = right_index {
        let Some(str_slice) = str.value.get(left_index..right) else {
            return errors::indexing_error();
        };
        return AllObjects::StringObj(StringObj {
            value: Rc::new(str_slice.to_string()),
        });
    }

    let mut chars = str.value.chars();
    let Some(ch) = chars.nth(left_index) else {
        return errors::indexing_error();
    };

    AllObjects::StringObj(StringObj {
        value: Rc::new(ch.to_string()),
    })
}

pub fn get_hash_map_value(m: &HashMapObj, key: &AllObjects) -> AllObjects {
    if let Some(v) = m.map.borrow().get(key) {
        return v.clone();
    }
    NULL
}
