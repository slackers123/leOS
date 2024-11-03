use tokenzier::{Token, Tokenizer};

pub mod tokenzier;

pub fn tokenize(src: String) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokenizer = Tokenizer::new(&chars);
    tokenizer.tokens()
}
