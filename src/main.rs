pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
fn main() {
    let user = whoami::username();
    print!("\x1B[2J\x1B[1;1H");
    println!(
        "Hello {}. This is the Monkey programming langugage\nFeel free to type in commands\n",
        user
    );

    repl::repl::start();
}
