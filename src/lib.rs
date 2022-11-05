// for testing stdout in print function
#![feature(internal_output_capture)]

mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;

pub use object::environment::Environment;
pub use repl::{execute_program, start_repl};
use std::{error::Error, fs, io::Write};

/// Read and execute the given input file
pub fn read_file<U: Write>(given_path: String, output: &mut U) -> Result<(), Box<dyn Error>> {
    let file_path = std::path::Path::new(&given_path);
    let content = fs::read(file_path)?;
    let input = String::from_utf8(content)?;
    execute_program(&input, output, Environment::new())?;

    Ok(())
}
