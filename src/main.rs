mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let mut l = Lexer::new("let v = 52;");

    for _ in 0..6 {
        let t = l.next_token();
        println!("{:?}", t);
    }
}
