extern crate rust_calculator;

use rust_calculator::calculator;
use rust_calculator::parser;
use rust_calculator::tokenizer;

use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let mut t = tokenizer::Tokenizer::new(format!("{}\0", input.trim_end()));
        t.tokenize().unwrap();

        let tokens = t.get_tokens();
        dbg!(&tokens);
        let mut parser = parser::Parser::new(tokens);
        parser.parse().unwrap();

        let tree = parser.get_tree();
        dbg!(tree);
        let n = calculator::calculate(tree.clone());
        println!("{}", n);
    }
}
