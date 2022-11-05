use crate::object::{AllObjects, ObjectType};

pub fn type_mismatch(left: &AllObjects, operator: &str, right: &AllObjects) -> AllObjects {
    AllObjects::new_error(format!(
        "type mismatch: {} {} {}",
        left.object_type(),
        operator,
        right.object_type()
    ))
}

pub fn unknown_operator(
    left: Option<&AllObjects>,
    operator: &str,
    right: &AllObjects,
) -> AllObjects {
    if let Some(l) = left {
        return AllObjects::new_error(format!(
            "unknown operator: {} {} {}",
            l.object_type(),
            operator,
            right.object_type()
        ));
    }
    AllObjects::new_error(format!(
        "unknown operator: {}{}",
        operator,
        right.object_type()
    ))
}

pub fn identifier_not_found(ident: &str) -> AllObjects {
    AllObjects::new_error(format!("identifier not found: {}", ident))
}

pub fn incorrect_arg_num(expected: usize, actual: usize) -> AllObjects {
    AllObjects::new_error(format!(
        "incorrect number of arguments supplied, expected: {}, supplied {}",
        expected, actual
    ))
}

pub fn argument_not_found(expected_arg: &str, expected_arg_type: ObjectType) -> AllObjects {
    AllObjects::new_error(format!(
        "expected argument {} of type {}",
        expected_arg, expected_arg_type
    ))
}

pub fn unexpected_argument_type(expected: ObjectType, actual: AllObjects) -> AllObjects {
    AllObjects::new_error(format!(
        "expected a {} argument, but received a {}",
        expected,
        actual.object_type()
    ))
}
