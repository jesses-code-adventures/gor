mod tests {
    #[cfg(test)]
    use std::fs;
    use gor::lexer::{lexer::Lexer, token_type::{Keyword, TokenKind}};

    #[test]
    fn test_simple() {
        let input = fs::read_to_string("tests/testfiles/simple.go").unwrap();
        let mut lexer = Lexer::new(&input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Package)));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Identifier));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Import)));
    }
}
