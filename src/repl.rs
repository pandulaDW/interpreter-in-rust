use crate::{lexer::Lexer, parser::program::Parser};
use std::io::{BufRead, Result, Write};

const PROMPT: &str = ">> ";

pub fn start_repl<T: BufRead, U: Write>(input: &mut T, output: &mut U) -> Result<()> {
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
        let mut p = Parser::new(l);

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
