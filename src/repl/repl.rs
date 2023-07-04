use std::io::{stdin, stdout, Write};

use crate::lexer::lexer;
const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        let mut lexer = lexer::Lexer::new(line);

        while let Ok(token) = lexer.next_token() {
            if token == lexer::Token::EOF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
