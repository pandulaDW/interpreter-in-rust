use crate::{
    evaluator,
    lexer::Lexer,
    object::Object,
    parser::{Parser, TRACING_ENABLED},
};
use clap::Parser as ClapParser;
use std::io::{BufRead, Result, Write};

const PROMPT: &str = ">> ";

/// The monkey programming language REPL (Read -> Evaluate -> Print -> Loop)
#[derive(ClapParser)]
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
        let mut p = Parser::new(l);
        let program = p.parse_program();

        if !p.errors.is_empty() {
            print_parser_errors(&p.errors);
            text.clear();
            continue;
        }
        program.statements.iter().for_each(|v| println!("{}", v));

        let evaluated = evaluator::eval(program.make_node());
        if let Some(e) = evaluated {
            println!("{}", e.inspect());
        }

        text.clear();
    }

    Ok(())
}

fn greet() {
    println!(
        "Hello {}!, This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
}

fn print_parser_errors(errors: &[String]) {
    println!("{}", MONKEY_FACE);
    println!("Woops! We ran into some monkey business here 🥴");
    println!("parser Errors:");
    errors.iter().for_each(|v| println!("\t- {}", v));
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
