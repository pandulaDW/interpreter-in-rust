use clap::Parser;

use crate::parser::TRACING_ENABLED;
use crate::{lexer::Lexer, parser::program};
use std::io::{BufRead, Result, Write};

const PROMPT: &str = ">> ";

/// The monkey programming language REPL (Read -> Evaluate -> Print -> Loop)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Enables tracing for parsing expressions
    #[clap(short, long, value_parser, default_value_t = false)]
    tracing: bool,
}

pub fn start_repl<T: BufRead, U: Write>(input: &mut T, output: &mut U) -> Result<()> {
    let args = Args::parse();
    unsafe {
        TRACING_ENABLED = args.tracing;
    }

    greet();

    let mut text = String::new();

    loop {
        write!(output, "{}", PROMPT)?;
        output.flush()?;

        input.read_line(&mut text)?;

        if text.trim() == r"\q" {
            writeln!(output, "bye")?;
            break;
        }

        let l = Lexer::new(&text);
        let mut p = program::Parser::new(l);

        let program = p.parse_program();

        if !p.errors.is_empty() {
            println!("encountered parser errors: {:?}", &p.errors);
        } else {
            for stmt in program.statements {
                println!("{}", stmt);
            }
        }

        text.clear();
    }

    Ok(())
}

fn greet() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
}
