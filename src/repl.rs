use crate::lexer::Lexer;
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

        let mut l = Lexer::new(&text);
        loop {
            let t = l.next_token();
            if !t.token_type.is_eof() {
                writeln!(output, "{:?}", t)?;
            } else {
                text.clear();
                break;
            }
        }
    }

    Ok(())
}
