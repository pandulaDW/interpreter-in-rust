mod lexer;
mod token;

use lexer::Lexer;
use std::io::{self, Write};

fn main() {
    const PROMPT: &'static str = ">> ";

    loop {
        let mut text = String::new();

        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut text).unwrap();

        if text.trim() == r"\q" {
            println!("bye");
            break;
        }

        let mut l = Lexer::new(&text);
        loop {
            let t = l.next_token();
            if !t.token_type.is_eof() {
                println!("{:?}", t);
            } else {
                break;
            }
        }
    }
}
