use super::{errors, helpers};
use crate::{
    ast::expressions::Identifier,
    object::{
        objects::{BuiltinFunctionObj, ParamsType},
        AllObjects, Object, ObjectType,
    },
    Environment,
};
use std::rc::Rc;
use std::{thread, time::Duration};

/// Return the associated builtin function based on the function name
pub fn get_builtin_function(ident: &Identifier) -> Option<AllObjects> {
    let func = match ident.value.as_str() {
        "len" => BuiltinFunctionObj {
            fn_name: "len".to_string(),
            parameters: ParamsType::Fixed(vec!["value".to_string()]),
            func: len,
        },
        "print" => BuiltinFunctionObj {
            fn_name: "print".to_string(),
            parameters: ParamsType::Variadic,
            func: print,
        },
        "push" => BuiltinFunctionObj {
            fn_name: "push".to_string(),
            parameters: ParamsType::Fixed(vec!["array".to_string(), "element".to_string()]),
            func: push,
        },
        "pop" => BuiltinFunctionObj {
            fn_name: "pop".to_string(),
            parameters: ParamsType::Fixed(vec!["array".to_string()]),
            func: pop,
        },
        "is_null" => BuiltinFunctionObj {
            fn_name: "is_null".to_string(),
            parameters: ParamsType::Fixed(vec!["value".to_string()]),
            func: is_null,
        },
        "insert" => BuiltinFunctionObj {
            fn_name: "insert".to_string(),
            parameters: ParamsType::Fixed(vec![
                "map".to_string(),
                "key".to_string(),
                "value".to_string(),
            ]),
            func: insert,
        },
        "delete" => BuiltinFunctionObj {
            fn_name: "delete".to_string(),
            parameters: ParamsType::Fixed(vec!["map".to_string(), "key".to_string()]),
            func: delete,
        },
        "sleep" => BuiltinFunctionObj {
            fn_name: "sleep".to_string(),
            parameters: ParamsType::Fixed(vec!["seconds".to_string()]),
            func: sleep,
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
        v => return errors::unexpected_argument_type("a STRING", v),
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

    if all_vars.is_empty() {
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
        v => return errors::unexpected_argument_type("an ARRAY", v),
    };

    // since all array borrows are temporary, this wouldn't cause a panic.
    array.elements.borrow_mut().push(element);

    helpers::NULL
}

/// Removes the last element from an array and returns it.
///
/// Returns null, if the array is empty
pub fn pop(env: Rc<Environment>) -> AllObjects {
    let array = get_argument("array", env);

    let array = match array {
        AllObjects::ArrayObj(v) => v,
        v => return errors::unexpected_argument_type("an ARRAY", v),
    };

    // since all array borrows are temporary, this wouldn't cause a panic.
    let popped = match array.elements.borrow_mut().pop() {
        Some(v) => v,
        None => helpers::NULL,
    };

    popped
}

/// Checks if the passed value is a null
pub fn is_null(env: Rc<Environment>) -> AllObjects {
    let is_null = matches!(get_argument("value", env), AllObjects::Null(_));
    helpers::get_bool_consts(is_null)
}

/// Inserts a key-value pair into the map.
///
/// If the map did not have this key present, Null is returned.
///
/// If the map did have this key present, the value is updated, and the old value is returned
pub fn insert(env: Rc<Environment>) -> AllObjects {
    let map_arg = get_argument("map", env.clone());
    let key = get_argument("key", env.clone());
    let value = get_argument("value", env);

    let m = match map_arg {
        AllObjects::HashMap(v) => v,
        v => return errors::unexpected_argument_type("a hash map", v),
    };

    if let Some(v) = m.map.borrow_mut().insert(key, value) {
        return v;
    }

    helpers::NULL
}

/// Removes a key from the map, returning the value at the key if the key was previously in the map and
/// returns Null otherwise
pub fn delete(env: Rc<Environment>) -> AllObjects {
    let map_arg = get_argument("map", env.clone());
    let key = get_argument("key", env.clone());

    let m = match map_arg {
        AllObjects::HashMap(v) => v,
        v => return errors::unexpected_argument_type("a hash map", v),
    };

    if let Some(v) = m.map.borrow_mut().remove(&key) {
        return v;
    }

    helpers::NULL
}

/// Puts the main thread to sleep for at least the specified amount of time given in seconds
pub fn sleep(env: Rc<Environment>) -> AllObjects {
    let seconds = match get_argument("seconds", env.clone()) {
        AllObjects::Integer(n) => n,
        v => return errors::unexpected_argument_type("an integer", v),
    };

    let Ok(seconds) = TryInto::<u64>::try_into(seconds.value) else {
        return errors::sleep_arg_error();
    };

    thread::sleep(Duration::from_secs(seconds));

    helpers::NULL
}

fn get_argument(arg_name: &str, env: Rc<Environment>) -> AllObjects {
    match env.get(arg_name) {
        Some(v) => v,
        None => errors::argument_not_found("value", ObjectType::String),
    }
}
