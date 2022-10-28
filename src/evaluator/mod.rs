mod eval;

pub use eval::eval;

#[cfg(test)]
mod tests {
    use super::test_helpers::*;

    #[test]
    fn test_eval_integer_expression() {
        let test_cases = [("5", 5), ("10", 10)];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let test_cases = [("true", true), ("false", false)];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_boolean_obj(evaluated, tc.1);
        }
    }

    #[test]
    fn test_bang_operator() {
        let test_cases = [
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_boolean_obj(evaluated, tc.1);
        }
    }
}

#[cfg(test)]
mod test_helpers {
    use super::eval::eval;
    use crate::{lexer::Lexer, object::AllObjects, parser};

    pub fn helper_test_eval(input: &str) -> Option<AllObjects> {
        let l = Lexer::new(input);
        let mut p = parser::Parser::new(l);
        let program = p.parse_program();

        eval(program.make_node())
    }

    pub fn helper_test_integer_obj(obj: Option<AllObjects>, expected: i64) {
        if let AllObjects::Integer(obj) = obj.expect(EXPECTED_OBJECT) {
            assert_eq!(obj.value, expected);
        } else {
            panic!("{}", EXPECTED_INT_OBJECT);
        }
    }

    pub fn helper_test_boolean_obj(obj: Option<AllObjects>, expected: bool) {
        if let AllObjects::Boolean(obj) = obj.expect(EXPECTED_OBJECT) {
            assert_eq!(obj.value, expected);
        } else {
            panic!("{}", EXPECTED_INT_OBJECT);
        }
    }

    const EXPECTED_OBJECT: &str = "expected an object";
    const EXPECTED_INT_OBJECT: &str = "expected an integer object";
}
