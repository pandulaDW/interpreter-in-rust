use crate::object::AllObjects;

pub fn type_mismatch(left: &AllObjects, operator: &str, right: &AllObjects) -> AllObjects {
    AllObjects::new_error(format!(
        "type mismatch: {:?} {} {:?}",
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
            "unknown operator: {:?} {} {:?}",
            l.object_type(),
            operator,
            right.object_type()
        ));
    }
    AllObjects::new_error(format!(
        "unknown operator: {}{:?}",
        operator,
        right.object_type()
    ))
}
