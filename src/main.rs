mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;

use std::io::{self, BufReader};
use std::{error::Error, fs};

use crate::object::environment::Environment;

fn main() {
    let mut args = std::env::args();
    if args.len() > 1 {
        let file_path = args.nth(1).unwrap();
        if let Err(e) = read_file(file_path) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    let mut reader = BufReader::new(io::stdin());
    let mut writer = io::stdout();

    if let Err(e) = repl::start_repl(&mut reader, &mut writer) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Read and execute the input file
fn read_file(given_path: String) -> Result<(), Box<dyn Error>> {
    let file_path = std::path::Path::new(&given_path);
    let content = fs::read(file_path)?;

    let input = String::from_utf8(content)?;
    repl::execute_program(&input, Environment::new());

    Ok(())
}
