mod ast;
mod lexer;
mod repl;

use std::io::{self, BufReader};
use whoami;

fn user_name() -> String {
    whoami::username()
}

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        user_name()
    );
    println!("Feel free to type in commands");

    let mut reader = BufReader::new(io::stdin());
    let mut writer = io::stdout();

    if let Err(e) = repl::start_repl(&mut reader, &mut writer) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
