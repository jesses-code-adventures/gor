use crate::primitives::position::Position;

pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct Statement {
    pub kind: StatementKind,
    pub position_start: Position,
    pub position_end: Position,
}

pub enum StatementKind {
    Expression(Expression),
}

pub struct Expression {
    pub kind: ExpressionKind,
    pub position_start: Position,
    pub position_end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Identifier(String),
    IntegerLiteral(String),
}

impl Expression {
    pub fn new(
        kind: ExpressionKind,
        position_start: Position,
        position_end: Position,
    ) -> Expression {
        Expression {
            kind,
            position_start,
            position_end,
        }
    }

    pub fn new_identifier(value: String, position: Position) -> Expression {
        Expression::new(
            ExpressionKind::Identifier(value),
            position,
            position,
        )
    }

    pub fn new_integer_literal(value: String, position: Position) -> Expression {
        Expression::new(
            ExpressionKind::IntegerLiteral(value),
            position,
            position,
        )
    }
}

mod tests {
    use super::*;
    #[test]
    fn expression_creation() {
        let position = Position::new(0, 0, 0);
        let expression = Expression::new_identifier("main".to_string(), position);
        assert_eq!(expression.kind, ExpressionKind::Identifier("main".to_string()));
        assert_eq!(expression.position_start, position);
        assert_eq!(expression.position_end, position);
    }
}
