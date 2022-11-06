use super::{errors, helpers};
use crate::{
    ast::expressions::Identifier,
    object::{
        objects::{BuiltinFunction, ParamsType},
        AllObjects, Object, ObjectType,
    },
    Environment,
};
use std::rc::Rc;

/// Return the associated builtin function based on the function name
pub fn get_builtin_function(ident: &Identifier) -> Option<AllObjects> {
    let func = match ident.value.as_str() {
        "len" => BuiltinFunction {
            fn_name: "len".to_string(),
            parameters: ParamsType::Fixed(vec!["value".to_string()]),
            func: len,
        },
        "print" => BuiltinFunction {
            fn_name: "print".to_string(),
            parameters: ParamsType::Variadic,
            func: print,
        },
        "push" => BuiltinFunction {
            fn_name: "push".to_string(),
            parameters: ParamsType::Fixed(vec!["array".to_string(), "element".to_string()]),
            func: push,
        },
        "pop" => BuiltinFunction {
            fn_name: "pop".to_string(),
            parameters: ParamsType::Fixed(vec!["array".to_string()]),
            func: pop,
        },
        _ => return None,
    };

    Some(AllObjects::BuiltinFunction(func))
}

/// Returns the length of a string, an array or a hashmap.
///
/// The function expects an argument called value, which must be one of the said types.
pub fn len(env: Rc<Environment>) -> AllObjects {
    let value = get_argument("value", env);

    let length = match value {
        AllObjects::StringObj(v) => v.value.len(),
        AllObjects::ArrayObj(v) => v.elements.borrow().len(),
        AllObjects::Error(_) => return value,
        v => return errors::unexpected_argument_type(ObjectType::String, v),
    };

    // panic of conversion from usize to i64 is highly unlikely
    let length = length.try_into().unwrap();

    helpers::get_int_object_for_value(length)
}

/// Takes a variable number of arguments and prints each one consecutively to the stdout with a single space separator.
///
/// If no arguments are provided, it will print a newline.
pub fn print(env: Rc<Environment>) -> AllObjects {
    let all_vars = env.all_vars();

    for (i, var) in all_vars.iter().enumerate() {
        let arg = match env.get(var) {
            Some(v) => v,
            None => return errors::identifier_not_found(var),
        };

        print!("{}", arg.inspect());
        if i != all_vars.len() - 1 {
            print!(" ");
        }
    }

    if env.all_vars().is_empty() {
        println!();
    }

    helpers::NULL
}

/// Appends an element to the back of the array
pub fn push(env: Rc<Environment>) -> AllObjects {
    let array = get_argument("array", env.clone());
    let element = get_argument("element", env);

    let array = match array {
        AllObjects::ArrayObj(v) => v,
        v => return errors::unexpected_argument_type(ObjectType::Array, v),
    };

    // since all array borrows are temporary, this wouldn't cause a panic.
    array.elements.borrow_mut().push(element);

    helpers::NULL
}

/// Removes the last element from a vector and returns it.
///
/// Returns null, if the array is empty
pub fn pop(env: Rc<Environment>) -> AllObjects {
    let array = get_argument("array", env);

    let array = match array {
        AllObjects::ArrayObj(v) => v,
        v => return errors::unexpected_argument_type(ObjectType::Array, v),
    };

    // since all array borrows are temporary, this wouldn't cause a panic.
    let popped = match array.elements.borrow_mut().pop() {
        Some(v) => v,
        None => helpers::NULL,
    };

    popped
}

fn get_argument(arg_name: &str, env: Rc<Environment>) -> AllObjects {
    match env.get(arg_name) {
        Some(v) => v,
        None => errors::argument_not_found("value", ObjectType::String),
    }
}
