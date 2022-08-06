mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let mut l = Lexer::new("let v = 5;");
    let token = l.next_token();

    println!("{:?}", token);
}
