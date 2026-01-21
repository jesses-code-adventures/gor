use crate::lexer::{lexer::Lexer, token::Token, token_type::TokenKind};
use crate::parser::{
    ast::{Expression, Program, Statement, StatementKind},
    errors::{ParserError, ParserErrorKind},
};

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
        self.errors.push(error);
        self.synchronize();
        return Ok(self.advance());
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut statements = Vec::new();
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        } else {
            while !matches!(self.peek().kind, Some(TokenKind::EOF)) {
                let statement_result = self.parse_statement();
                if let Err(error) = statement_result {
                    self.errors.push(error);
                    continue;
                }
                statements.push(statement_result.unwrap());
            }
            Ok(Program { statements })
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression()?;
        let semicolon = self.expect_token(TokenKind::Semicolon)?;
        Ok(Statement {
            kind: StatementKind::Expression(expression.clone()),
            position_start: expression.position_start,
            position_end: semicolon.position,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.peek().kind {
            Some(TokenKind::Identifier) => {
                let identifier = self.expect_token(TokenKind::Identifier)?;
                Ok(Expression::new_identifier(
                    identifier.value.clone(),
                    identifier.position,
                ))
            }
            Some(TokenKind::IntegerLiteral) => {
                let integer = self.expect_token(TokenKind::IntegerLiteral)?;
                Ok(Expression::new_integer_literal(
                    integer.value.clone(),
                    integer.position,
                ))
            }
            _ => Err(ParserError::new(
                ParserErrorKind::UnexpectedToken(self.peek().value.clone()),
                self.peek().position,
            )),
        }
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
    }
}
