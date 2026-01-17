mod tokeniser;
mod parser;
use crate::tokeniser::lexer;

fn main(){
    lexer(include_str!("./test_code.txt").to_string()).unwrap();
}
