use interpreter_lib::read_file;

#[test]
fn input_works() {
    let test_cases = vec![
        ("basic_functions.mok", "1054"),
        ("if_statements.mok", "8750"),
        ("higher_order_functions.mok", "hello Vimu! bye Vimu!"),
        ("array_map.mok", "[11, 21, 31, 41, 51]"),
        ("array_filter.mok", "[4, 10, 120, 90]"),
        ("array_reduce.mok", "38"),
    ];
    let base_path = "tests/testfiles";

    for tc in test_cases {
        let mut output: Vec<u8> = Vec::new();
        if let Err(e) = read_file(format!("{}/{}", base_path, tc.0), &mut output) {
            panic!("{}", e);
        }

        let result = match String::from_utf8(output) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        let trimmed = result.trim();
        assert_eq!(tc.1, trimmed);
    }
}
