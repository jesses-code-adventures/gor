// A lexer
pub struct Lexer {
    token: String
}

impl Lexer {
    pub fn new(token: String) -> Lexer {
        Lexer {
            token: token
        }
    }

    pub fn next(&mut self) -> String {
        self.token.clone()
    }
}
