use crate::position::Position;
use crate::token_type::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: Option<TokenKind>,
    pub value: String,
    pub position: Position,
}

impl Token {
    pub fn new(value: &str, position: Position) -> Token {
        Token {
            kind: TokenKind::from_str(value),
            value: value.to_string(),
            position,
        }
    }

    pub fn new_with_kind(kind: TokenKind, value: &str, position: Position) -> Token {
        Token {
            kind: Some(kind),
            value: value.to_string(),
            position,
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
