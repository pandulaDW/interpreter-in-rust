use interpreter_lib::{read_file, start_repl};
use std::io::{self, BufReader};

fn main() {
    let mut writer = io::stdout();

    let mut args = std::env::args();
    if args.len() > 1 {
        let file_path = args.nth(1).unwrap();
        if let Err(e) = read_file(file_path, &mut writer) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    let mut reader = BufReader::new(io::stdin());
    if let Err(e) = start_repl(&mut reader, &mut writer) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
