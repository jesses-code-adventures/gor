use crate::primitives::position::Position;
use crate::lexer::token_type::Operator;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub position_start: Position,
    pub position_end: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    // 10 + 20 * 30
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    // foo
    Identifier(String),
    // 10
    IntegerLiteral(String),
    // "hello there"
    StringLiteral(String),
    // foo(bar, baz)
    FunctionCall {
        name: Box<Expression>, // Can be simple identifier or field access
        arguments: Vec<Expression>,
    },
    // foo.bar
    FieldAccess {
        object: Box<Expression>, // fmt
        field: String,           // Println
    },
    // (expr)
    Parenthesized(Box<Expression>), // (expr)
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
        operator: Operator,
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

