mod lexer;
mod parser;
mod ast;
mod env;
mod interpreter;

use std::fs;
use std::env as std_env;

fn main() {
    let args: Vec<String> = std_env::args().collect();

    if args.len() < 2 {
        eprintln!(" interpreter program.mgo");
        return;
    }

    let filename = &args[1];

    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!(" Failed to read file '{}': {}", filename, err);
            return;
        }
    };

    let tokens = lexer::tokenize(&code);
    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse();

    let mut env = env::Env::default();
    interpreter::eval(&stmts, &mut env);
}
