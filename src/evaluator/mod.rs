mod builtins;
mod errors;
mod eval;
mod helpers;

pub use eval::eval;

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use crate::object::AllObjects;
    use std::io;
    use std::sync::Arc;

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
            (r#" "foobar" == "foobar" "#, true),
            (r#" "foo" != "baz" "#, true),
            (r#" "x" < "y" "#, true),
            (
                r#" "a slightly long text" == "not so slightly long text" "#,
                false,
            ),
            ("is_null(5)", false),
            ("let x = null; is_null(x)", true),
            ("let x = 5; !is_null(x)", true),
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
    fn test_if_expression_scope() {
        let input = "if (1 < 2) { 
           let x = 10;
        }
        x - 10;
        ";
        let evaluated = helper_test_eval(input);
        helper_test_error(evaluated, "identifier not found: x");

        let input = "if (1 > 2) { 
            let x = 10;
         } else {
            return 20 + x;
         }
         ";
        let evaluated = helper_test_eval(input);
        helper_test_error(evaluated, "identifier not found: x");
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

    #[test]
    fn test_error_handling() {
        let test_cases = [
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) {
                if (10 > 1) {
                  return true + false;
                }
              return 1; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            match evaluated.expect(EXPECTED_ERROR) {
                AllObjects::Error(e) => assert_eq!(e.message, tc.1),
                _ => panic!("{}", EXPECTED_ERROR),
            }
        }
    }

    #[test]
    fn test_let_statements() {
        let test_cases = [
            ("let a = 5;", 5),
            ("let a = 5; a;", 5),
            ("let a = 5; let a = 10; a;", 10),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }
    }

    #[test]
    fn test_null_literal() {
        let input = "let x = null; x;";
        let evaluated = helper_test_eval(input);
        helper_test_null(evaluated);
    }

    #[test]
    fn test_string_literals() {
        let test_cases = [
            ("let a = \"foobar\";a;", "foobar"),
            ("return \"baz\"", "baz"),
            ("let x = \"iron\"; return x;", "iron"),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_string_literal(evaluated, tc.1);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; }";
        let evaluated = helper_test_eval(input).expect(EXPECTED_ERROR);
        let AllObjects::Function(v) = evaluated else {
            panic!("{}", EXPECTED_FUNCTION);
        };
        assert_eq!(v.parameters.len(), 1);
        assert_eq!(v.parameters[0].to_string(), "x");
        assert_eq!(v.body.to_string(), "(x + 2)");
    }

    #[test]
    fn test_function_application() {
        let test_cases = [
            ("let identity = fn(x) { x; }; identity(5);", 5),
            (
                "let identity = fn(x) { return x; puts(48); }; identity(5);",
                5,
            ),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(6, 10));", 26),
            ("fn(x) { x + 2; }(5)", 7),
            (
                "
            let a = 20;
            let add_using_outer_var = fn() {
                let c = a + 12;
                return c;
            }
            add_using_outer_var();
            ",
                32,
            ),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }
    }

    #[test]
    fn test_closures() {
        let input = "
        let newAdder = fn(x) {
            return fn(y) { x + y };
            };
          let addTwo = newAdder(2);
          addTwo(3);";
        let evaluated = helper_test_eval(input);
        helper_test_integer_obj(evaluated, 5);
    }

    #[test]
    fn test_string_concatenation() {
        let input = r#""foo" + " " + "bar""#;
        let evaluated = helper_test_eval(input);
        helper_test_string_literal(evaluated, "foo bar");
    }

    #[test]
    fn test_len() {
        let test_cases = [
            (r#" len("foobar"); "#, 6),
            ("len([1, 2, true, 10, 20, false, \"foo\"])", 7),
            ("len([])", 0),
            ("len(\"\")", 0),
        ];
        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }

        let input = r#" len(12); "#;
        let evaluated = helper_test_eval(input);
        helper_test_error(
            evaluated,
            "expected a STRING argument, but received an INTEGER",
        );
    }

    #[test]
    fn test_print_function() {
        io::set_output_capture(Some(Default::default()));

        let input = r#" print(12, 34, "foobar\n", true); "#;
        _ = helper_test_eval(input);

        let captured = std::io::set_output_capture(None);
        let captured = captured.unwrap();
        let captured = Arc::try_unwrap(captured).unwrap();
        let captured = captured.into_inner().unwrap();
        let captured = String::from_utf8(captured).unwrap();

        assert_eq!(captured, "12 34 foobar\n true");
    }

    #[test]
    fn test_arrays() {
        let input = r#"
            let x = [3 * 4, "foo", true];
            push(x, false);            
            push(x, "bar");
            pop(x);
            x;
        "#;

        let binding = match helper_test_eval(input).expect(EXPECTED_OBJECT) {
            AllObjects::ArrayObj(v) => v,
            _ => panic!("{}", EXPECTED_ARRAY),
        };

        let mut array = binding.elements.borrow_mut();

        assert_eq!(array.len(), 4);
        helper_test_integer_obj(Some(array.remove(0)), 12);
        helper_test_string_literal(Some(array.remove(0)), "foo");
        helper_test_boolean_obj(Some(array.remove(0)), true);
        helper_test_boolean_obj(Some(array.remove(0)), false);
    }

    #[test]
    fn test_indexing() {
        let test_cases = [
            ("[1, 2, 3][0]", 1),
            ("[1, 2, 3][1]", 2),
            ("[1, 2, 3][2]", 3),
            ("let i = 0; [1][i];", 1),
            ("[1, 2, 3][1 + 1];", 3),
            ("let i = 0; [1][i];", 1),
            ("let myArray = [1, 2, 3]; myArray[2];", 3),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                6,
            ),
            ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2),
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }

        let input = "[1, 2, 3][3]";
        let evaluated = helper_test_eval(input);
        helper_test_error(evaluated, "list index out of range");

        let input = r#"let x = "foobar"; x[0] + "bar"[2]"#;
        let evaluated = helper_test_eval(input);
        helper_test_string_literal(evaluated, "fr");
    }

    #[test]
    fn test_array_pop() {
        let input = r#"
            let x = [12, "foo", true];
            let v = pop(x);
            v
        "#;

        let evaluated = helper_test_eval(input);
        helper_test_boolean_obj(evaluated, true);

        let input = r#"let x = []; pop(x);"#;
        let evaluated = helper_test_eval(input);
        helper_test_null(evaluated);
    }

    #[test]
    fn test_assignment_expression() {
        let test_cases = [
            ("let x = 12; x = 25;", 25),
            ("let x = 9; x = x + 1;", 10),
            ("let x = 12; if(true) { x = 13; }; x;", 13),
            ("let x = 12; if(true) { let x = 13; x = x + 1;}; x;", 12), // testing scope
        ];

        for tc in test_cases {
            let evaluated = helper_test_eval(tc.0);
            helper_test_integer_obj(evaluated, tc.1);
        }

        let input = "y = 10";
        let evaluated = helper_test_eval(input);
        helper_test_error(evaluated, "identifier not found: y");
    }

    #[test]
    fn test_range_expressions() {
        let input = "let x = [12,4,5,6,1]; x[1:4];";

        let arr_obj = match helper_test_eval(input).expect(EXPECTED_OBJECT) {
            AllObjects::ArrayObj(v) => v,
            _ => panic!("{}", EXPECTED_ARRAY),
        };

        let mut array = arr_obj.elements.borrow_mut();
        assert_eq!(array.len(), 3);
        helper_test_integer_obj(Some(array.remove(0)), 4);
        helper_test_integer_obj(Some(array.remove(0)), 5);
        helper_test_integer_obj(Some(array.remove(0)), 6);

        let input = r#" "foobar"[0:3] "#;
        let evaluated = helper_test_eval(input);
        helper_test_string_literal(evaluated, "foo");
    }

    #[test]
    fn test_hash_maps() {
        let input = r#"let m = {"foo": 4, "bar": 5}; m["foo"] + m["bar"]"#;
        let evaluated = helper_test_eval(input);
        helper_test_integer_obj(evaluated, 9);

        let input = r#"let m = {"foo": 4}; m["bar"]"#;
        let evaluated = helper_test_eval(input);
        helper_test_null(evaluated);

        let input = r#"let key=true; let m = {key: 40 * 2}; m[key]"#;
        let evaluated = helper_test_eval(input);
        helper_test_integer_obj(evaluated, 80);
    }

    #[test]
    fn test_map_builtins() {
        let input = "
            let m = {};
            let i = 0;
            while (i < 10) {
                insert(m, i, 10 * i);
                i = i + 1;
            }
            m
        ";
        let map = match helper_test_eval(input).expect(EXPECTED_HASH_MAP) {
            AllObjects::HashMap(v) => v,
            _ => panic!("{}", EXPECTED_HASH_MAP),
        };
        assert_eq!(map.map.borrow().len(), 10);

        let input = r#"let m = {"foo": 4, "bar": 5}; delete(m, "bar"); m;"#;
        let map = match helper_test_eval(input).expect(EXPECTED_HASH_MAP) {
            AllObjects::HashMap(v) => v,
            _ => panic!("{}", EXPECTED_HASH_MAP),
        };
        assert_eq!(map.map.borrow().len(), 1);
    }

    #[test]
    fn test_while_statement() {
        let input = "
            let i = 1;
            let x = 0;

            while(i < 6) {
                x = x + 10;
                i = i + 1;
            }

            x;
        ";
        let evaluated = helper_test_eval(input);
        helper_test_integer_obj(evaluated, 50);
    }
}

#[cfg(test)]
mod test_helpers {
    use super::eval::eval;
    use crate::{
        lexer::Lexer,
        object::{environment::Environment, AllObjects},
        parser,
    };

    pub fn helper_test_eval(input: &str) -> Option<AllObjects> {
        let l = Lexer::new(input);
        let mut p = parser::Parser::new(l);
        let program = p.parse_program();
        let new_env = Environment::new();

        eval(program.make_node(), new_env)
    }

    pub fn helper_test_integer_obj(obj: Option<AllObjects>, expected: i64) {
        let AllObjects::Integer(obj) = obj.expect(EXPECTED_OBJECT) else {
            panic!("{}", EXPECTED_INT_OBJECT);
        };
        assert_eq!(obj.value, expected);
    }

    pub fn helper_test_string_literal(obj: Option<AllObjects>, expected: &str) {
        let AllObjects::StringObj(obj) = obj.expect(EXPECTED_OBJECT) else{
            panic!("{}", EXPECTED_STRING_OBJECT);
        };
        assert_eq!(*obj.value, expected);
    }

    pub fn helper_test_boolean_obj(obj: Option<AllObjects>, expected: bool) {
        let AllObjects::Boolean(obj) = obj.expect(EXPECTED_OBJECT) else {
            panic!("{}", EXPECTED_BOOLEAN_OBJECT);
        };
        assert_eq!(obj.value, expected);
    }

    pub fn helper_test_null(obj: Option<AllObjects>) {
        match obj.unwrap() {
            AllObjects::Null(_) => {}
            _ => panic!("{}", EXPECTED_NULL),
        }
    }

    pub fn helper_test_error(obj: Option<AllObjects>, message: &str) {
        match obj.unwrap() {
            AllObjects::Error(e) => assert_eq!(e.message, message),
            _ => panic!("{}", EXPECTED_ERROR),
        }
    }

    pub const EXPECTED_ERROR: &str = "expected an error object";
    pub const EXPECTED_OBJECT: &str = "expected an object";
    pub const EXPECTED_FUNCTION: &str = "expected a function";
    pub const EXPECTED_ARRAY: &str = "expected an array";
    pub const EXPECTED_HASH_MAP: &str = "expected a hash map";
    const EXPECTED_INT_OBJECT: &str = "expected an integer object";
    const EXPECTED_BOOLEAN_OBJECT: &str = "expected a boolean object";
    const EXPECTED_STRING_OBJECT: &str = "expected a string object";
    const EXPECTED_NULL: &str = "expected null";
}
