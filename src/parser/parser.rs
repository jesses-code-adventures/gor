use crate::lexer::{lexer::Lexer, token::Token, token_type::TokenKind};
use crate::parser::{
    ast::{Expression, Program, Statement},
    errors::{ParserError, ParserErrorKind},
};
use crate::primitives::position::Position;

pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let current_token = Token::new_before_start();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn advance(&mut self) -> &Token {
        self.current_token = self.peek_token.clone();

        if self.current_token.kind != Some(TokenKind::EOF) {
            self.peek_token = self.lexer.next_token();
        } else {
            self.peek_token = Token::new_with_kind(TokenKind::EOF, "", self.current_token.position);
        }

        return &self.current_token;
    }

    fn peek(&self) -> &Token {
        &self.peek_token
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<&Token, ParserError> {
        if self.peek().kind == Some(kind) {
            return Ok(self.advance());
        }
        let error = ParserError::new(
            ParserErrorKind::UnexpectedToken(self.peek().value.clone()),
            self.peek().position,
        );
        self.errors.push(error.clone());
        self.synchronize();
        return Err(error);
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut statements = Vec::new();
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        } else {
            while !matches!(self.peek().kind, Some(TokenKind::EOF)) {
                match self.parse_statement() {
                    Ok(statement) => {
                        statements.push(statement);
                    }
                    Err(error) => {
                        self.errors.push(error);
                        // Try to recover by synchronizing to next statement boundary
                        self.synchronize();
                        // Skip the current problematic token to avoid infinite loop
                        if !matches!(self.peek().kind, Some(TokenKind::EOF)) {
                            self.advance();
                        }
                    }
                }
            }
            Ok(Program { statements })
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek().kind {
            Some(TokenKind::Package) => self.parse_package_declaration(),
            Some(TokenKind::Import) => self.parse_import_declaration(),
            Some(TokenKind::Func) => self.parse_function_declaration(),
            _ => {
                // Default to expression statement
                self.parse_expression_statement()
            }
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression()?;
        let start_position = expression.position_start;
        let end_position = self.handle_semicolon_insertion()?;
        Ok(Statement::new_expression_statement(
            expression,
            start_position,
            end_position,
        ))
    }

    fn parse_package_declaration(&mut self) -> Result<Statement, ParserError> {
        let package_token = self.expect_token(TokenKind::Package)?;
        let package_pos = package_token.position;
        let name_token = self.expect_token(TokenKind::Identifier)?;
        let name_value = name_token.value.clone();
        let end_position = self.handle_semicolon_insertion()?;

        Ok(Statement::new_package_declaration(
            name_value,
            package_pos,
            end_position,
        ))
    }

    fn parse_import_declaration(&mut self) -> Result<Statement, ParserError> {
        let import_token = self.expect_token(TokenKind::Import)?;
        let import_pos = import_token.position;
        let path_token = self.expect_token(TokenKind::StringLiteral)?;
        let path_value = path_token.value.clone();
        let end_position = self.handle_semicolon_insertion()?;

        Ok(Statement::new_import_declaration(
            path_value,
            import_pos,
            end_position,
        ))
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, ParserError> {
        let func_token = self.expect_token(TokenKind::Func)?;
        let func_pos = func_token.position;
        let name_token = self.expect_token(TokenKind::Identifier)?;
        let func_name = name_token.value.clone();

        self.expect_token(TokenKind::LeftParen)?;
        // TODO: Implement parameter parsing
        self.expect_token(TokenKind::RightParen)?;

        self.expect_token(TokenKind::LeftBrace)?;
        let mut body_statements = Vec::new();

        while !matches!(self.peek().kind, Some(TokenKind::RightBrace)) {
            if matches!(self.peek().kind, Some(TokenKind::EOF)) {
                return Err(ParserError::new(
                    ParserErrorKind::UnexpectedToken(
                        "Expected '}' to close function body".to_string(),
                    ),
                    self.peek().position,
                ));
            }
            body_statements.push(self.parse_statement()?);
        }

        let right_brace = self.expect_token(TokenKind::RightBrace)?;
        let end_pos = right_brace.position;

        Ok(Statement::new_function_declaration(
            func_name,
            Vec::new(), // No parameters for now
            body_statements,
            func_pos,
            end_pos,
        ))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.peek().kind {
            Some(TokenKind::Identifier) => self.parse_identifier_expression(),
            Some(TokenKind::IntegerLiteral) => {
                let integer = self.expect_token(TokenKind::IntegerLiteral)?;
                Ok(Expression::new_integer_literal(
                    integer.value.clone(),
                    integer.position,
                ))
            }
            Some(TokenKind::StringLiteral) => {
                let string = self.expect_token(TokenKind::StringLiteral)?;
                Ok(Expression::new_string_literal(
                    string.value.clone(),
                    string.position,
                ))
            }
            _ => Err(ParserError::new(
                ParserErrorKind::UnexpectedToken(self.peek().value.clone()),
                self.peek().position,
            )),
        }
    }

    /// Parse identifier-based expressions (identifier, field access, function calls, etc)
    fn parse_identifier_expression(&mut self) -> Result<Expression, ParserError> {
        let identifier_token = self.expect_token(TokenKind::Identifier)?;
        let mut expression =
            Expression::new_identifier(identifier_token.value.clone(), identifier_token.position);

        loop {
            match self.peek().kind {
                Some(TokenKind::Dot) => {
                    // Field access: obj.field
                    self.advance(); // consume the dot
                    let field_token = self.expect_token(TokenKind::Identifier)?;
                    let start_pos = expression.position_start;
                    let end_pos = field_token.position;

                    expression = Expression::new_field_access(
                        expression,
                        field_token.value.clone(),
                        start_pos,
                        end_pos,
                    );
                }
                Some(TokenKind::LeftParen) => {
                    // Function call: expr(args)
                    let start_pos = expression.position_start;
                    self.advance(); // consume the '('

                    let mut arguments = Vec::new();

                    if !matches!(self.peek().kind, Some(TokenKind::RightParen)) {
                        loop {
                            arguments.push(self.parse_expression()?);

                            if matches!(self.peek().kind, Some(TokenKind::Comma)) {
                                self.advance(); // consume comma
                            } else {
                                break;
                            }
                        }
                    }

                    let right_paren = self.expect_token(TokenKind::RightParen)?;
                    let end_pos = right_paren.position;

                    expression =
                        Expression::new_function_call(expression, arguments, start_pos, end_pos);
                }
                _ => {
                    // No more chaining, return the expression
                    break;
                }
            }
        }

        Ok(expression)
    }

    fn handle_semicolon_insertion(&mut self) -> Result<Position, ParserError> {
        if self.peek().kind == Some(TokenKind::Semicolon) {
            let semicolon = self.advance();
            Ok(semicolon.position)
        } else if self.is_end_of_line() {
            Ok(self.current_token.position)
        } else {
            Err(ParserError::new(
                ParserErrorKind::UnexpectedToken(format!(
                    "Expected ';' to separate statements on same line, got '{}'",
                    self.peek().value
                )),
                self.peek().position,
            ))
        }
    }

    fn is_end_of_line(&mut self) -> bool {
        if matches!(self.peek().kind, Some(TokenKind::EOF)) {
            return true;
        }

        self.lexer.had_newline_before_current_token()
    }

    fn synchronize(&mut self) {
        while !matches!(
            self.peek().kind,
            Some(TokenKind::EOF) | Some(TokenKind::Semicolon)
        ) {
            self.advance();
        }
    }
}

mod tests {
    #[cfg(test)]
    mod tests {
        use crate::{
            lexer::token_type::TokenKind,
            parser::{
                ast::{Expression, StatementKind},
                parser::Parser,
            },
            primitives::position::Position,
        };

        #[test]
        fn parser_parse_program() {
            let input = "identifier;";
            let mut parser = Parser::new(input);
            let program = parser.parse().unwrap();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0].kind,
                StatementKind::Expression(Expression::new_identifier(
                    "identifier".to_string(),
                    Position::new(1, 0, 10)
                ))
            );
        }

        #[test]
        fn parser_initialization() {
            let input = "func main";
            let parser = Parser::new(input);

            // Parser should start "before" the first token
            assert_eq!(parser.current_token.kind, Some(TokenKind::BeforeStart));
            assert_eq!(parser.peek_token.kind, Some(TokenKind::Func));
            assert_eq!(parser.errors.len(), 0);
        }

        #[test]
        fn advance_simple() {
            let input = "func main";
            let mut parser = Parser::new(input);

            // First advance should get "func"
            let token = parser.advance();
            assert_eq!(token.kind, Some(TokenKind::Func));
            assert_eq!(token.value, "func");

            // Peek should now be "main"
            assert_eq!(parser.peek().kind, Some(TokenKind::Identifier));
            assert_eq!(parser.peek().value, "main");
        }

        #[test]
        fn advance_to_eof() {
            let input = "func";
            let mut parser = Parser::new(input);

            // First advance gets "func"
            parser.advance();
            assert_eq!(parser.current_token.kind, Some(TokenKind::Func));

            // Second advance gets EOF
            parser.advance();
            assert_eq!(parser.current_token.kind, Some(TokenKind::EOF));

            // Peek should also be EOF
            assert_eq!(parser.peek().kind, Some(TokenKind::EOF));
        }

        #[test]
        fn expect_token_success() {
            let input = "func main";
            let mut parser = Parser::new(input);

            // Expect "func" - should succeed
            let token = parser.expect_token(TokenKind::Func).unwrap();
            assert_eq!(token.kind, Some(TokenKind::Func));
            assert_eq!(token.value, "func");
            assert_eq!(parser.errors.len(), 0);

            // Current token should now be "func", peek should be "main"
            assert_eq!(parser.current_token.kind, Some(TokenKind::Func));
            assert_eq!(parser.peek().kind, Some(TokenKind::Identifier));
        }

        #[test]
        fn expect_token_failure() {
            let input = "func main";
            let mut parser = Parser::new(input);

            // Expect "var" but get "func" - should fail and synchronize
            let _token = parser.expect_token(TokenKind::Var);
            assert_eq!(parser.errors.len(), 1);

            // Should have synchronized and advanced past the error
            assert!(parser.current_token.kind.is_some());
        }

        #[test]
        fn synchronize_to_semicolon() {
            let input = "func main ( ) ;";
            let mut parser = Parser::new(input);

            // Expect something wrong to trigger synchronization
            let _ = parser.expect_token(TokenKind::Var); // Wrong token

            // Should have synchronized to before the semicolon
            assert_eq!(parser.errors.len(), 1);
        }

        #[test]
        fn synchronize_to_eof() {
            let input = "func main";
            let mut parser = Parser::new(input);

            // Expect wrong token to trigger synchronization
            let _ = parser.expect_token(TokenKind::Var); // Wrong token

            // Should synchronize to EOF since there's no semicolon
            assert_eq!(parser.errors.len(), 1);
        }

        #[test]
        fn peek_consistency() {
            let input = "func main ( )";
            let mut parser = Parser::new(input);

            // Peek should be consistent
            assert_eq!(parser.peek().kind, Some(TokenKind::Func));
            assert_eq!(parser.peek().kind, Some(TokenKind::Func)); // Multiple peeks

            // Advance and check again
            parser.advance();
            assert_eq!(parser.peek().kind, Some(TokenKind::Identifier));
            assert_eq!(parser.peek().kind, Some(TokenKind::Identifier)); // Multiple peeks
        }

        #[test]
        fn empty_input() {
            let input = "";
            let parser = Parser::new(input);

            // Should handle empty input gracefully
            assert_eq!(parser.peek().kind, Some(TokenKind::EOF));
            assert_eq!(parser.errors.len(), 0);
        }

        #[test]
        fn whitespace_handling() {
            let input = "  func   main  ";
            let mut parser = Parser::new(input);

            // Whitespace should be ignored
            assert_eq!(parser.peek().kind, Some(TokenKind::Func));

            parser.advance();
            assert_eq!(parser.current_token.kind, Some(TokenKind::Func));
            assert_eq!(parser.peek().kind, Some(TokenKind::Identifier));
        }

        #[test]
        fn multiple_advances() {
            let input = "func main ( ) { }";
            let mut parser = Parser::new(input);

            // Test sequence of advances
            parser.advance(); // func
            assert_eq!(parser.current_token.kind, Some(TokenKind::Func));

            parser.advance(); // main
            assert_eq!(parser.current_token.kind, Some(TokenKind::Identifier));

            parser.advance(); // (
            assert_eq!(parser.current_token.kind, Some(TokenKind::LeftParen));

            parser.advance(); // )
            assert_eq!(parser.current_token.kind, Some(TokenKind::RightParen));

            parser.advance(); // {
            assert_eq!(parser.current_token.kind, Some(TokenKind::LeftBrace));

            parser.advance(); // }
            assert_eq!(parser.current_token.kind, Some(TokenKind::RightBrace));

            parser.advance(); // EOF
            assert_eq!(parser.current_token.kind, Some(TokenKind::EOF));
        }

        #[test]
        fn semicolon_insertion_single_statement_no_semicolon() {
            let input = "println";
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(
                result.is_ok(),
                "Should parse single statement without semicolon"
            );
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn semicolon_insertion_single_statement_with_semicolon() {
            let input = "println;";
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(
                result.is_ok(),
                "Should parse single statement with semicolon"
            );
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn semicolon_insertion_multiple_statements_no_semicolon_error() {
            let input = "println println";
            let mut parser = Parser::new(input);
            let _ = parser.parse();
            assert!(
                !parser.errors.is_empty(),
                "Should have errors for multiple statements without semicolon"
            );
        }

        #[test]
        fn semicolon_insertion_multiline_with_newlines() {
            let input = "println\nprintln\nprintln";
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(
                result.is_ok(),
                "Should parse multiple lines without semicolons"
            );
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 3);
        }

        #[test]
        fn parse_string_literal() {
            let input = r#""Hello, World!""#;
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(result.is_ok(), "Should parse string literal");
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn parse_simple_function_call() {
            let input = r#"println("hello")"#;
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(result.is_ok(), "Should parse simple function call");
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn parse_method_call() {
            let input = r#"fmt.Println("hello")"#;
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(result.is_ok(), "Should parse method call");
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn parse_package_declaration() {
            let input = "package main";
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(result.is_ok(), "Should parse package declaration");
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn parse_import_declaration() {
            let input = r#"import "fmt""#;
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(result.is_ok(), "Should parse import declaration");
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn parse_simple_function_declaration() {
            let input = "func main() {\n    println(\"hello\")\n}";
            let mut parser = Parser::new(input);
            let result = parser.parse();
            assert!(result.is_ok(), "Should parse function declaration");
            let program = result.unwrap();
            assert_eq!(program.statements.len(), 1);
        }

        #[test]
        fn parse_complete_hello_world() {
            let input = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#;
            let mut parser = Parser::new(input);
            let result = parser.parse();
            if let Err(ref errors) = result {
                println!("Parse errors: {:?}", errors);
            }
            if !parser.errors.is_empty() {
                println!("Parser errors: {:?}", parser.errors);
            }
            assert!(result.is_ok(), "Should parse complete Hello World program");
            let program = result.unwrap();
            println!("Parsed {} statements", program.statements.len());
            assert_eq!(program.statements.len(), 3); // package, import, func
        }
    }
}
