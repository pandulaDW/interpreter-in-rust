mod eval;

pub use eval::eval;

#[cfg(test)]
mod tests {
    use super::test_helpers::*;

    #[test]
    fn test_eval_integer_expression() {
        let test_cases = [
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-1024", -1024),
            ("1 + 2", 3),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let test_cases = [
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

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

    #[test]
    fn test_if_expressions() {
        let test_cases = [
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);

            if tc.1.is_some() {
                helper_test_integer_obj(evaluated, tc.1.unwrap());
            } else {
                helper_test_null(evaluated);
            }
        }
    }

    #[test]
    fn test_return_statement() {
        let test_cases = [
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                "if (10 > 1) {
                if (10 > 1) {
                return 10;
              }
             return 1;",
                10,
            ),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
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

    pub fn helper_test_null(obj: Option<AllObjects>) {
        match obj.unwrap() {
            AllObjects::Null(_) => {}
            _ => panic!("{}", EXPECTED_NULL),
        }
    }

    const EXPECTED_OBJECT: &str = "expected an object";
    const EXPECTED_INT_OBJECT: &str = "expected an integer object";
    const EXPECTED_NULL: &str = "expected null";
}
