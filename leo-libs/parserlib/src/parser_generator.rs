pub struct ParserGenerator<'a> {
    pub src: &'a str,
    index: usize,
}

impl<'a> ParserGenerator<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, index: 0 }
    }

    pub fn generate(&mut self) {}
}
