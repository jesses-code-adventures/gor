use crate::position::Position;
use crate::token_type::TokenType;

pub struct Token {
    kind: TokenType,
    position: Position,
}

impl Token {
    pub fn new(value: &str, line: u32, column_start: u32, column_end: u32) -> Option<Token> {
        match TokenType::from_str(value) {
            Some(kind) => Some(Token {
                kind,
                position: Position::new(line, column_start, column_end),
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_tokenizes() {
        let token = Token::new("break", 1, 1, 5).unwrap();
        assert_eq!(token.kind, TokenType::Break);
    }
}
