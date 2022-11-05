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

pub fn get_builtin_function(ident: &Identifier) -> Option<AllObjects> {
    let func = match ident.value.as_str() {
        "len" => BuiltinFunction {
            fn_name: ident.value.clone(),
            parameters: ParamsType::Fixed(vec!["value".to_string()]),
            func: len,
        },
        "print" => BuiltinFunction {
            fn_name: ident.value.clone(),
            parameters: ParamsType::Variadic,
            func: print,
        },
        _ => return None,
    };

    Some(AllObjects::BuiltinFunction(func))
}

/// Returns the length of a string, an array or a hashmap.
///
/// The function expects an argument called value, which must be one of the said types.
pub fn len(env: Rc<Environment>) -> AllObjects {
    let value = match env.get("value") {
        Some(v) => v,
        None => return errors::argument_not_found("value", ObjectType::String),
    };

    let str_obj = match value {
        AllObjects::StringObj(v) => v,
        v => return errors::unexpected_argument_type(ObjectType::String, v),
    };

    // panic of conversion from usize to i64 is highly unlikely
    let length = str_obj.value.len().try_into().unwrap();

    helpers::get_int_object_for_value(length)
}

/// Takes a variable number of arguments and prints each one consecutively to the stdout with a single space separator.
///
/// If no arguments are provided, it will print a newline.
pub fn print(env: Rc<Environment>) -> AllObjects {
    for var in env.all_vars() {
        let arg = match env.get(&var) {
            Some(v) => v,
            None => return errors::identifier_not_found(&var),
        };

        print!("{} ", arg.inspect());
    }

    if env.all_vars().is_empty() {
        println!();
    }

    helpers::NULL
}
