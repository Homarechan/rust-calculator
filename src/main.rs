extern crate rust_calculator;

use rust_calculator::parser;
use rust_calculator::tokenizer;

fn main() {
    let expr: String = "1 + 2 * 3\0".to_string();
    let mut t = tokenizer::Tokenizer::new(expr);
    t.tokenize().unwrap();
    let tokens = t.get_tokens();
    dbg!(&tokens);
    let mut parser = parser::Parser::new(tokens);
    parser.parse().unwrap();
    dbg!(parser.get_tree());
}
