use std::env;
use std::fs;

mod lexer;
mod parser;
mod interpreter;
mod error;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: gaex <arquivo.gx>");
        return;
    }

    let filename = &args[1];

    let code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Não foi possível ler '{}': {}", filename, e);
            return;
        }
    };

    let mut lexer = Lexer::new(&code);

    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);

    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Erro de parsing: {}", e);
            return;
        }
    };

    let mut interpreter = Interpreter::new();

    if let Err(e) = interpreter.interpretar(&stmts) {
        eprintln!("Erro de execução: {}", e);
    }
}
