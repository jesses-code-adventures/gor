use crate::primitives::{errors::lexer::LexerError, position::Position};

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub position: Position,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, position: Position) -> ParserError {
        ParserError { kind, position }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error at {:?}: {}", self.position, self.kind)
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
    LexerError(LexerError),
    UnexpectedToken(String),
    NotAPrimaryExpression(String),
    NotImplemented,
}

impl std::fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorKind::LexerError(error) => write!(f, "{}", error),
            ParserErrorKind::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParserErrorKind::NotAPrimaryExpression(token) => write!(f, "Not a primary expression: {}", token),
            ParserErrorKind::NotImplemented => write!(f, "Not implemented"),
        }
    }
}
