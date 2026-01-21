use crate::primitives::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub position: Position,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, position: Position) -> LexerError {
        LexerError { kind, position }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error at {:?}: {}", self.position, self.kind)
    }
}

impl std::error::Error for LexerError {}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorKind {
    IncompleteToken(String),
    UnexpectedToken(String),
    UnterminatedString(String),
    UnterminatedRune(String),
}

impl std::fmt::Display for LexerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorKind::IncompleteToken(token) => write!(f, "Incomplete token: {}", token),
            LexerErrorKind::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            LexerErrorKind::UnterminatedString(token) => {
                write!(f, "Unterminated string: {}", token)
            }
            LexerErrorKind::UnterminatedRune(token) => write!(f, "Unterminated rune: {}", token),
        }
    }
}
