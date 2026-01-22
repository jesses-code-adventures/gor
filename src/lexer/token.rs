use crate::lexer::token_type::{TokenKind, Keyword};
use crate::primitives::position::Position;

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

    pub fn new_before_start() -> Token {
        Token {
            kind: Some(TokenKind::BeforeStart),
            value: "".to_string(),
            position: Position::new(0, 0, 0),
        }
    }

    /// if this token precedes a newline outside a string, should the parser insert a semicolon?
    /// (according to the formal syntax -> https://go.dev/ref/spec#Semicolons )
    //
    // from the docs, as of the time of writing this...
    // an identifier
    // an integer, floating-point, imaginary, rune, or string literal
    // one of the keywords break, continue, fallthrough, or return
    // one of the operators and punctuation ++, --, ), ], or }
    pub fn should_insert_semicolon(&self) -> bool {
        match self.kind {
            Some(TokenKind::Identifier)
            | Some(TokenKind::IntegerLiteral)
            | Some(TokenKind::FloatLiteral)
            | Some(TokenKind::RuneLiteral)
            | Some(TokenKind::StringLiteral)
            | Some(TokenKind::Keyword(Keyword::Break))
            | Some(TokenKind::Keyword(Keyword::Continue))
            | Some(TokenKind::Keyword(Keyword::Fallthrough))
            | Some(TokenKind::Keyword(Keyword::Return))
            | Some(TokenKind::PlusPlus)
            | Some(TokenKind::MinusMinus)
            | Some(TokenKind::RightParen)
            | Some(TokenKind::RightBracket)
            | Some(TokenKind::RightBrace) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_tokenizes() {
        let token = Token::new("func", Position::new(0, 0, 3));
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Func)));
    }
}
