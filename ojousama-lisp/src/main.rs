use std::io::{self, Write};

use crate::{lexer::Lexer, token::TokenKind};

mod lexer;
mod token;

fn main() {
    print!(">> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let mut lexer = Lexer::new(&input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", &token);
        if token.kind == TokenKind::EOF {
            break;
        }
    }
}
