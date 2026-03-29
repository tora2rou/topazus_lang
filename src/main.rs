mod parser;
mod tokeniser;
use crate::tokeniser::lexer;

fn main() {
    lexer(include_str!("./test_code.txt").to_string()).unwrap();
}
