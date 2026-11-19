
mod tokenizer;
mod parser;

use std::io::{self, Write};
use parser::Parser;

fn main() {
    println!("CALCULADORA");
    println!("Digite 'exit' para sair.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        // Tokenizer
        let tokens = match tokenizer::tokenize(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Erro: {err}");
                continue;
            }
        };

        // Parser
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(result) => println!("= {result}"),
            Err(err) => println!("Erro: {err}"),
        }
    }
}
