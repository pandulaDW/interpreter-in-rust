use crate::{
    evaluator,
    lexer::Lexer,
    object::{environment::Environment, Object},
    parser::{Parser, TRACING_ENABLED},
};
use clap::Parser as ClapParser;
use std::{
    io::{self, BufRead, Write},
    rc::Rc,
};

const PROMPT: &str = ">> ";

/// The monkey programming language REPL (Read -> Evaluate -> Print -> Loop)
#[derive(ClapParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Enables tracing for parsing expressions
    #[clap(short, long, value_parser, default_value_t = false)]
    tracing: bool,
}

pub fn start_repl<T: BufRead, U: Write>(input: &mut T, output: &mut U) -> io::Result<()> {
    let args = Args::parse();
    unsafe {
        TRACING_ENABLED = args.tracing;
    }
    greet(output)?;

    let mut text = String::new();
    let program_env = Environment::new();

    loop {
        write!(output, "{}", PROMPT)?;
        output.flush()?;

        input.read_line(&mut text)?;

        let trimmed = text.trim();
        if trimmed == r"\q" {
            writeln!(output, "bye")?;
            break;
        }

        if !trimmed.is_empty() {
            execute_program(&text, output, program_env.clone())?;
        }

        text.clear();
    }

    Ok(())
}

fn greet<U: Write>(output: &mut U) -> io::Result<()> {
    writeln!(
        output,
        "Hello {}!, This is the Monkey programming language!",
        whoami::username()
    )?;
    writeln!(output, "Feel free to type in commands")?;
    Ok(())
}

fn write_parser_errors<U: Write>(errors: &[String], output: &mut U) -> io::Result<()> {
    writeln!(output, "{}", MONKEY_FACE)?;
    writeln!(output, "Woops! We ran into some monkey business here 🥴")?;
    writeln!(output, "parser Errors:")?;
    for e in errors {
        writeln!(output, "\t- {}", e)?;
    }
    Ok(())
}

const MONKEY_FACE: &str = r#"
            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

pub fn execute_program<U: Write>(
    text: &str,
    output: &mut U,
    program_env: Rc<Environment>,
) -> io::Result<()> {
    let l = Lexer::new(text);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    if !p.errors.is_empty() {
        write_parser_errors(&p.errors, output)?;
        return Ok(());
    }

    let evaluated = evaluator::eval(program.make_node(), program_env);
    match evaluated {
        Some(e) if !e.is_null() => {
            writeln!(output, "{}", e.inspect())?;
        }
        Some(_) | None => writeln!(output)?,
    }

    Ok(())
}
