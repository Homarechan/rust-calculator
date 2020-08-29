extern crate rust_calculator;

use rust_calculator::tokenizer;

fn main() {
    let expr: String = "1 + 2 * 3 ** (3 - 2) ^ 4\0".to_string();
    let mut t = tokenizer::Tokenizer::new(expr);
    t.tokenize().unwrap();
    dbg!(t.get_tokens());
}
