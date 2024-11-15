use tokenzier::{Token, Tokenizer};

pub mod element;
pub mod html_namespace;
pub mod mathml;
pub mod node;
pub mod parser;
pub mod tokenzier;

pub fn tokenize(src: String) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    let tokenizer = Tokenizer::new(&chars);
    tokenizer.flatten().collect()
}
