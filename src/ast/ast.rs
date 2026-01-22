use crate::ast::statement::Statement;

pub struct Program {
    pub statements: Vec<Statement>,
}

mod tests {
    #[cfg(test)]
    use crate::{
        ast::expression::{Expression, ExpressionKind},
        primitives::position::Position,
    };

    #[test]
    fn expression_creation() {
        let position = Position::new(0, 0, 0);
        let expression = Expression::new_identifier("main".to_string(), position);
        assert_eq!(
            expression.kind,
            ExpressionKind::Identifier("main".to_string())
        );
        assert_eq!(expression.position_start, position);
        assert_eq!(expression.position_end, position);
    }
}
