use interpreter_lib::read_file;

#[test]
fn input_works() {
    let paths = vec!["tests/testfiles/basic_functions.mok"];

    for p in paths {
        if let Err(e) = read_file(p.to_string()) {
            panic!("{}", e);
        }
    }
}
