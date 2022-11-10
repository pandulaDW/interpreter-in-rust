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

pub fn unexpected_argument_type(expected: &str, actual: AllObjects) -> AllObjects {
    let expected = expected.to_string();
    let actual = actual.object_type().to_string();

    AllObjects::new_error(format!(
        "expected {} argument, but received {} {}",
        expected,
        a_or_an(&actual),
        actual
    ))
}

pub fn indexing_error() -> AllObjects {
    AllObjects::new_error("list index out of range".to_string())
}

pub fn incorrect_index_argument() -> AllObjects {
    AllObjects::new_error("list index argument should be a positive integer".to_string())
}

const A: &str = "a";
const AN: &str = "an";

fn a_or_an(word: &str) -> &'static str {
    match word.chars().next() {
        Some(c) => match c {
            'A' | 'E' | 'I' | 'O' | 'U' => AN,
            _ => A,
        },
        None => A,
    }
}
