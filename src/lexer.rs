pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { source }
    }

    pub fn scan_tokens(&self) -> Vec<String> {
        self.source
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }
}
