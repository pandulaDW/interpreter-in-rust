mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;

use std::io::{self, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = io::stdout();

    if let Err(e) = repl::start_repl(&mut reader, &mut writer) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
