use super::{errors, helpers::get_int_object_for_value};
use crate::{
    ast::expressions::Identifier,
    object::{objects::BuiltinFunction, AllObjects, ObjectType},
    Environment,
};
use std::rc::Rc;

pub fn get_builtin_function(ident: &Identifier) -> Option<AllObjects> {
    let func = match ident.value.as_str() {
        "len" => len,
        _ => return None,
    };

    Some(AllObjects::BuiltinFunction(BuiltinFunction {
        fn_name: ident.value.clone(),
        parameters: vec!["value".to_string()],
        func,
    }))
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

    get_int_object_for_value(length)
}
