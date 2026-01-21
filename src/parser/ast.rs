use crate::primitives::position::Position;

pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct Statement {
    pub kind: StatementKind,
    pub position: Position,
}

pub enum StatementKind {
    Expression(Expression),
}

pub struct Expression {
    pub kind: ExpressionKind,
    pub position: Position,
}

pub enum ExpressionKind {
    Identifier(String),
}
