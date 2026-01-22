use crate::primitives::{position::Position};

#[derive(Debug, Clone, PartialEq)]
pub struct ProgrammingError {
    pub kind: ProgrammingErrorKind,
    pub position: Position,
}

impl ProgrammingError {
    pub fn new(kind: ProgrammingErrorKind, position: Position) -> ProgrammingError {
        ProgrammingError { kind, position }
    }
}

impl std::fmt::Display for ProgrammingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error at {:?}: {}", self.position, self.kind)
    }
}

impl std::error::Error for ProgrammingError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammingErrorKind {
    LogicError(String)
}

impl std::fmt::Display for ProgrammingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgrammingErrorKind::LogicError(error) => write!(f, "Logically impossible execution: {}", error),
        }
    }
}
