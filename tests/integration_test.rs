use interpreter_lib::read_file;

#[test]
fn input_works() {
    let test_cases = vec![
        ("tests/testfiles/basic_functions.mok", "1054"),
        ("tests/testfiles/if_statements.mok", "8750"),
        (
            "tests/testfiles/higher_order_functions.mok",
            "hello Vimu! bye Vimu!",
        ),
    ];

    for tc in test_cases {
        let mut output: Vec<u8> = Vec::new();
        if let Err(e) = read_file(tc.0.to_string(), &mut output) {
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
