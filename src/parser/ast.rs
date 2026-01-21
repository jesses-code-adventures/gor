use crate::primitives::position::Position;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub position_start: Position,
    pub position_end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Expression(Expression),
    PackageDeclaration(String),
    ImportDeclaration(String),
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>, // TODO: There should be a Parameter type
        body: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub position_start: Position,
    pub position_end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Identifier(String),
    IntegerLiteral(String),
    StringLiteral(String),
    FunctionCall {
        name: Box<Expression>, // Can be simple identifier or field access
        arguments: Vec<Expression>,
    },
    FieldAccess {
        object: Box<Expression>, // fmt
        field: String,           // Println
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Parenthesized(Box<Expression>), // (expr)
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    // Arithmetic
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
}

impl BinaryOperator {
    /// Get operator precedence (higher number = higher precedence)
    pub fn precedence(&self) -> u8 {
        match self {
            // Comparison (lowest)
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual => 1,

            // Addition level
            BinaryOperator::Add | BinaryOperator::Subtract => 2,

            // Multiplication level (highest)
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => 3,
        }
    }
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
        Expression::new(ExpressionKind::Identifier(value), position, position)
    }

    pub fn new_integer_literal(value: String, position: Position) -> Expression {
        Expression::new(ExpressionKind::IntegerLiteral(value), position, position)
    }

    pub fn new_string_literal(value: String, position: Position) -> Expression {
        Expression::new(ExpressionKind::StringLiteral(value), position, position)
    }

    pub fn new_function_call(
        name: Expression,
        arguments: Vec<Expression>,
        start_pos: Position,
        end_pos: Position,
    ) -> Expression {
        Expression::new(
            ExpressionKind::FunctionCall {
                name: Box::new(name),
                arguments,
            },
            start_pos,
            end_pos,
        )
    }

    pub fn new_field_access(
        object: Expression,
        field: String,
        start_pos: Position,
        end_pos: Position,
    ) -> Expression {
        Expression::new(
            ExpressionKind::FieldAccess {
                object: Box::new(object),
                field,
            },
            start_pos,
            end_pos,
        )
    }

    pub fn new_binary(
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
        start_pos: Position,
        end_pos: Position,
    ) -> Expression {
        Expression::new(
            ExpressionKind::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
            start_pos,
            end_pos,
        )
    }

    pub fn new_parenthesized(
        expression: Expression,
        start_pos: Position,
        end_pos: Position,
    ) -> Expression {
        Expression::new(
            ExpressionKind::Parenthesized(Box::new(expression)),
            start_pos,
            end_pos,
        )
    }
}

impl Statement {
    pub fn new(kind: StatementKind, position_start: Position, position_end: Position) -> Statement {
        Statement {
            kind,
            position_start,
            position_end,
        }
    }

    pub fn new_package_declaration(
        name: String,
        start_pos: Position,
        end_pos: Position,
    ) -> Statement {
        Statement::new(StatementKind::PackageDeclaration(name), start_pos, end_pos)
    }

    pub fn new_import_declaration(
        path: String,
        start_pos: Position,
        end_pos: Position,
    ) -> Statement {
        Statement::new(StatementKind::ImportDeclaration(path), start_pos, end_pos)
    }

    pub fn new_function_declaration(
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
        start_pos: Position,
        end_pos: Position,
    ) -> Statement {
        Statement::new(
            StatementKind::FunctionDeclaration {
                name,
                parameters,
                body,
            },
            start_pos,
            end_pos,
        )
    }

    pub fn new_expression_statement(
        expression: Expression,
        start_pos: Position,
        end_pos: Position,
    ) -> Statement {
        Statement::new(StatementKind::Expression(expression), start_pos, end_pos)
    }
}

mod tests {
    use super::*;
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
