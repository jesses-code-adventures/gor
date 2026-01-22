use crate::primitives::position::Position;
use crate::ast::expression::Expression;

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

