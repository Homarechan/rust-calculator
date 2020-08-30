extern crate rust_calculator;

use rust_calculator::calculator;
use rust_calculator::parser;
use rust_calculator::tokenizer;

fn main() {
    let expr: String = "1 + 2 * (3 - 1)\0".to_string();
    let mut t = tokenizer::Tokenizer::new(expr);
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
