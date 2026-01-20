use crate::position::Position;
use crate::token_type::TokenKind;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token {
    pub kind: Option<TokenKind>,
    pub position: Position,
}

impl Token {
    pub fn new(value: &str, position: Position) -> Token {
        Token {
            kind: TokenKind::from_str(value),
            position: position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_tokenizes() {
        let token = Token::new("func", Position::new(0, 0, 3));
        assert_eq!(token.kind, Some(TokenKind::Func));
    }
}
