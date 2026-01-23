use crate::lexer::token::Token;
use crate::lexer::token_type::TokenKind;
use crate::primitives::{
    errors::lexer::{LexerError, LexerErrorKind},
    position::Position,
};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String, // TODO: this should be a stream or a &str but i cbf to deal with lifetimes
    current_position: usize,
    anchor: usize,
    errors: Vec<LexerError>,
    is_parsing_string: bool,
    is_parsing_rune: bool,
    newline_before_current_token: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            current_position: 0,
            anchor: 0,
            errors: Vec::new(),
            is_parsing_string: false,
            is_parsing_rune: false,
            newline_before_current_token: false,
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            match self.next() {
                Some(ch) => match ch {
                    ch if is_whitespace(ch) && !self.is_parsing_string && !self.is_parsing_rune => {
                        self.handle_whitespace();
                        continue;
                    }
                    ch if is_whitespace(ch) && self.is_parsing_rune => {
                        self.errors.push(LexerError::new(
                            LexerErrorKind::UnterminatedRune(
                                self.proposed_token(false).to_string(),
                            ),
                            self.current_token_position(),
                        ));
                        self.is_parsing_rune = false;
                        self.anchor = self.current_position;
                        return Token::new("", self.current_token_position());
                    }
                    '"' => {
                        if self.is_parsing_string {
                            // End of string - include the closing quote
                            return self.finalize_string();
                        } else {
                            // Start of string
                            self.is_parsing_string = true;
                            self.anchor = self.current_position - 1; // Include the opening quote -
                                                                     // we've already called
                                                                     // next(), so we need to go
                                                                     // back a char
                            continue;
                        }
                    }
                    '\'' => {
                        if self.is_parsing_rune {
                            // End of rune - include the closing quote
                            return self.finalize_rune();
                        } else {
                            // Start of rune
                            self.is_parsing_rune = true;
                            self.anchor = self.current_position - 1; // Include the opening quote -
                                                                     // we've already called
                                                                     // next(), so we need to go
                                                                     // back a char
                            continue;
                        }
                    }
                    ch if self.is_parsing_string => {
                        if ch == '\\' {
                            self.next();
                        }
                        continue;
                    }
                    ch if self.is_parsing_rune => {
                        if ch == '\\' {
                            // Consume the next character as well (escape sequence)
                            self.next();
                        }
                        continue;
                    }
                    ch if is_symbol(ch) => {
                        if let Some(token) = self.handle_symbol_char() {
                            return token;
                        }
                        continue;
                    }
                    _ => {
                        if let Some(token) = self.handle_word() {
                            return token;
                        }
                        continue;
                    }
                },
                None => {
                    if self.is_parsing_string {
                        self.errors.push(LexerError::new(
                            LexerErrorKind::UnterminatedString(
                                self.proposed_token(false).to_string(),
                            ),
                            self.current_token_position(),
                        ));
                        self.is_parsing_string = false;
                        self.anchor = self.current_position;
                        return Token::new("", self.current_token_position());
                    }

                    if self.is_parsing_rune {
                        self.errors.push(LexerError::new(
                            LexerErrorKind::UnterminatedRune(
                                self.proposed_token(false).to_string(),
                            ),
                            self.current_token_position(),
                        ));
                        self.is_parsing_rune = false;
                        self.anchor = self.current_position;
                        return Token::new("", self.current_token_position());
                    }

                    return Token::new_with_kind(TokenKind::EOF, "", self.current_token_position());
                }
            }
        }
    }

    pub fn peek_tokens(&mut self, lookahead: usize) -> Vec<Token> {
        let mut tokens = Vec::new();
        let current_position = self.current_position;
        let anchor = self.anchor;
        let is_parsing_string = self.is_parsing_string;
        let is_parsing_rune = self.is_parsing_rune;
        for _ in 0..lookahead {
            let token = self.next_token();
            tokens.push(token);
        }
        self.current_position = current_position;
        self.anchor = anchor;
        self.is_parsing_string = is_parsing_string;
        self.is_parsing_rune = is_parsing_rune;
        tokens
    }

    fn handle_symbol_char(&mut self) -> Option<Token> {
        let symbol_pos = self.current_position - 1;
        if self.anchor < symbol_pos {
            // Check what type of characters we have pending
            let pending_value = &self.input[self.anchor..symbol_pos];

            // Only separate if the pending characters are not symbols
            // (i.e., we're transitioning from word to symbol, not symbol to symbol)
            if !pending_value.chars().all(is_symbol) {
                // Move current_position back to the symbol so it will be reprocessed
                // NOTE: this seems bad - we should create a word token when we
                // peek during word processing, rather than needing to mess with
                // the current_position
                self.current_position = symbol_pos;

                // Create word token
                let word_token = match TokenKind::from_str(pending_value) {
                    Some(_) => Token::new(
                        pending_value,
                        Position::new(self.current_line(), self.anchor, symbol_pos),
                    ),
                    None => {
                        self.errors.push(LexerError::new(
                            LexerErrorKind::UnexpectedToken(pending_value.to_string()),
                            Position::new(self.current_line(), self.anchor, symbol_pos),
                        ));
                        Token::new(
                            "",
                            Position::new(self.current_line(), self.anchor, symbol_pos),
                        )
                    }
                };

                // Set anchor for the symbol
                self.anchor = symbol_pos;
                return Some(word_token);
            }
        }

        if let Some(token) = self.handle_symbol() {
            return Some(token);
        }
        None
    }

    fn handle_word(&mut self) -> Option<Token> {
        let value = self.proposed_token(false);
        match self.tokenize(value) {
            Ok(Some(token)) => match token.kind {
                None => return None,
                _ => {
                    self.anchor = self.current_position;
                    return Some(token);
                }
            },
            Ok(None) => {
                return None;
            }
            Err(error) => {
                self.errors.push(error);
                self.anchor = self.current_position;
                return Some(Token::new("", self.current_token_position()));
            }
        }
    }

    fn handle_symbol(&mut self) -> Option<Token> {
        let value = self.proposed_token(false);
        match self.tokenize(value) {
            Ok(Some(token)) => {
                self.anchor = self.current_position;
                Some(token)
            }
            Ok(None) => None,
            Err(error) => {
                self.errors.push(error);
                self.anchor = self.current_position;
                Some(Token::new("", self.current_token_position()))
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.current_position >= self.input.len() {
            return None;
        }
        let c = self.input.chars().nth(self.current_position);
        self.current_position += 1;
        return c;
    }

    fn peek(&self) -> Option<char> {
        if self.current_position >= self.input.len() {
            return None;
        }
        let c = self.input.chars().nth(self.current_position);
        return c;
    }

    fn peek_is_whitespace(&self) -> bool {
        match self.peek() {
            Some(c) => is_whitespace(c),
            None => true, // EOF should be treated as whitespace boundary
        }
    }

    /// returns Some(token) if the token is tokenized, None if it is incomplete, err if there is an
    /// incomplete or unexpected full token
    fn tokenize(&self, value: &str) -> Result<Option<Token>, LexerError> {
        match TokenKind::from_str(value) {
            Some(_) => {
                if let Some(next_c) = self.peek() {
                    if !self.peek_is_whitespace() {
                        let longer = value.to_string() + &next_c.to_string();
                        if TokenKind::could_match(&longer) {
                            return Ok(None); // Continue accumulating
                        }
                    }
                }
                return Ok(Some(Token::new(value, self.current_token_position())));
            }
            None => {
                if !self.peek_is_whitespace() && TokenKind::could_match(value) {
                    return Ok(None);
                }

                // At a boundary but no valid token - this is an error
                return Err(LexerError::new(
                    LexerErrorKind::UnexpectedToken(value.to_string()),
                    self.current_token_position(),
                ));
            }
        };
    }

    fn finalize_string(&mut self) -> Token {
        self.is_parsing_string = false;
        let _string_content = &self.input[self.anchor..self.current_position];

        let token = Token::new_with_kind(
            TokenKind::StringLiteral,
            _string_content,
            self.current_token_position(),
        );

        self.anchor = self.current_position;
        token
    }

    fn finalize_rune(&mut self) -> Token {
        self.is_parsing_rune = false;
        let _rune_content = &self.input[self.anchor..self.current_position];

        let token = Token::new_with_kind(
            TokenKind::RuneLiteral,
            _rune_content,
            self.current_token_position(),
        );

        self.anchor = self.current_position;
        token
    }

    fn current_token_position(&self) -> Position {
        Position::new(self.current_line(), self.anchor, self.current_position)
    }

    fn handle_whitespace(&mut self) {
        // Check if the current character is a newline
        if let Some(ch) = self.input.chars().nth(self.current_position - 1) {
            if ch == '\n' {
                self.newline_before_current_token = true;
            }
        }
        self.anchor = self.current_position;
    }

    fn proposed_token(&self, already_iterated: bool) -> &str {
        &self.input[self.anchor..self.current_position - if already_iterated { 1 } else { 0 }]
    }

    fn current_line(&self) -> usize {
        let end = self.current_position.min(self.input.len());
        self.input[0..end].split('\n').count()
    }

    /// Check if a newline was encountered before the current token and reset the flag
    pub fn had_newline_before_current_token(&mut self) -> bool {
        let had_newline = self.newline_before_current_token;
        self.newline_before_current_token = false;
        had_newline
    }

    pub fn errors(&self) -> &[LexerError] {
        &self.errors
    }
}

fn is_symbol(c: char) -> bool {
    matches!(
        c,
        '+' | '-'
            | '*'
            | '/'
            | '%'
            | '&'
            | '|'
            | '^'
            | '<'
            | '>'
            | '='
            | '!'
            | '.'
            | ':'
            | ','
            | ';'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
    )
}

fn is_whitespace(c: char) -> bool {
    matches!(c, '\n' | '\t' | '\r' | ' ')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token_type::{Keyword, Operator};
    #[test]
    fn simple_statement() {
        let input = r#"j := i++"#;
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Identifier));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::ColonEqual));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Identifier));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::PlusPlus));
    }

    #[test]
    fn character_cutting_bug_demonstration() {
        let input = "hello world test";
        let mut lexer = Lexer::new(input);

        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = lexer.next_token();
            let mut should_break = false;
            if let Some(TokenKind::EOF) = token.kind {
                should_break = true;
            }
            tokens.push(token);
            if should_break {
                break;
            }
        }

        assert_eq!(lexer.errors.len(), 0);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, Some(TokenKind::Identifier));
        assert_eq!(tokens[1].kind, Some(TokenKind::Identifier));
        assert_eq!(tokens[2].kind, Some(TokenKind::Identifier));
        assert_eq!(tokens[0].value, "hello");
        assert_eq!(tokens[1].value, "world");
        assert_eq!(tokens[2].value, "test");
        assert_eq!(tokens[3].kind, Some(TokenKind::EOF));
    }

    #[test]
    fn token_separation_for_main_function() {
        let input = "main()";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        println!(
            "Token 1: {:?}, value: '{}', errors: {}",
            token1.kind,
            token1.value,
            lexer.errors.len()
        );
        assert_eq!(token1.kind, Some(TokenKind::Identifier));
        assert_eq!(lexer.errors.len(), 0);

        let token2 = lexer.next_token();
        println!("Token 2: {:?}, value: '{}'", token2.kind, token2.value);
        assert_eq!(token2.kind, Some(TokenKind::LeftParen));

        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Some(TokenKind::RightParen));

        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Some(TokenKind::EOF));
    }

    #[test]
    fn string_to_symbol_transition() {
        let input = "hello+world-test";
        let mut lexer = Lexer::new(input);

        // Should tokenize as: hello (error), +, world (error), -, test (error)
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::Identifier));
        assert_eq!(lexer.errors.len(), 0);

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::Operator(Operator::Plus)));

        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Some(TokenKind::Identifier));
        assert_eq!(lexer.errors.len(), 0);

        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Some(TokenKind::Operator(Operator::Minus)));

        let token5 = lexer.next_token();
        assert_eq!(token5.kind, Some(TokenKind::Identifier));

        let token6 = lexer.next_token();
        assert_eq!(token6.kind, Some(TokenKind::EOF));
    }

    #[test]
    fn basic_function() {
        let input = r#"func main() {
fmt.Println("Hello, World!")
}
f"#;
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Func)));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Identifier));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LeftParen));
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::RightParen));
    }

    #[test]
    fn func_start() {
        let input = "func";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Func)));
    }

    #[test]
    fn break_start() {
        let input = "break";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Break)));
    }

    #[test]
    fn case_start() {
        let input = "case";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Case)));
    }

    #[test]
    fn chan_start() {
        let input = "chan";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Chan)));
    }

    #[test]
    fn const_start() {
        let input = "const";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Const)));
    }

    #[test]
    fn continue_start() {
        let input = "continue";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Continue)));
    }

    #[test]
    fn default_start() {
        let input = "default";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Default)));
    }

    #[test]
    fn defer_start() {
        let input = "defer";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Defer)));
    }

    #[test]
    fn else_start() {
        let input = "else";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Else)));
    }

    #[test]
    fn fallthrough_start() {
        let input = "fallthrough";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Fallthrough)));
    }

    #[test]
    fn for_start() {
        let input = "for";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::For)));
    }

    #[test]
    fn go_start() {
        let input = "go";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Go)));
    }

    #[test]
    fn goto_start() {
        let input = "goto";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Goto)));
    }

    #[test]
    fn if_start() {
        let input = "if";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::If)));
    }

    #[test]
    fn import_start() {
        let input = "import";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Import)));
    }

    #[test]
    fn unterminated_rune_error() {
        let input = r#"'hello world"#;
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, None);

        assert_eq!(lexer.errors.len(), 1);
        // The error should be for an unterminated rune (invalid due to whitespace)
    }

    #[test]
    fn unterminated_rune_no_whitespace() {
        let input = r#"'abc"#; // No closing quote, no whitespace
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, None);

        assert_eq!(lexer.errors.len(), 1);
        // The error should be for an unterminated rune (EOF reached)
    }

    #[test]
    fn map_start() {
        let input = "map";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Map)));
    }

    #[test]
    fn package_start() {
        let input = "package";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Package)));
    }

    #[test]
    fn range_start() {
        let input = "range";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Range)));
    }

    #[test]
    fn return_start() {
        let input = "return";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Return)));
    }

    #[test]
    fn simple_string_parsing() {
        let input = r#""hello world""#;
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::StringLiteral));

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::EOF));

        // Should have no errors for a valid string
        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn string_with_escape_sequences() {
        let input = r#""hello \"quoted\" text\n""#;
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::StringLiteral));

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::EOF));

        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn unterminated_string_error() {
        let input = r#""hello world"#;
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, None);

        assert_eq!(lexer.errors.len(), 1);
        // The error should be for an unterminated string
    }

    #[test]
    fn string_mixed_with_other_tokens() {
        let input = r#"func main() { fmt.Println("Hello, World!") }"#;
        let mut lexer = Lexer::new(input);

        // func
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::Keyword(Keyword::Func)));

        // main (identifier)
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::Identifier));

        // (
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Some(TokenKind::LeftParen));

        // )
        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Some(TokenKind::RightParen));

        // {
        let token5 = lexer.next_token();
        assert_eq!(token5.kind, Some(TokenKind::LeftBrace));

        // fmt (identifier)
        let token6 = lexer.next_token();
        assert_eq!(token6.kind, Some(TokenKind::Identifier));

        // .
        let token7 = lexer.next_token();
        assert_eq!(token7.kind, Some(TokenKind::Dot));

        // Println (identifier)
        let token8 = lexer.next_token();
        assert_eq!(token8.kind, Some(TokenKind::Identifier));

        // (
        let token9 = lexer.next_token();
        assert_eq!(token9.kind, Some(TokenKind::LeftParen));

        // "Hello, World!" (string literal)
        let token10 = lexer.next_token();
        assert_eq!(token10.kind, Some(TokenKind::StringLiteral));

        // )
        let token11 = lexer.next_token();
        assert_eq!(token11.kind, Some(TokenKind::RightParen));

        // }
        let token12 = lexer.next_token();
        assert_eq!(token12.kind, Some(TokenKind::RightBrace));

        // EOF
        let token13 = lexer.next_token();
        assert_eq!(token13.kind, Some(TokenKind::EOF));

        // Should have no errors since all identifiers are valid
        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn simple_rune_parsing() {
        let input = r#"'a'"#;
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::RuneLiteral));

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::EOF));

        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn rune_with_escape_sequence() {
        let input = r#"'\n'"#;
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::RuneLiteral));

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::EOF));

        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn mixed_strings_and_runes() {
        let input = r#"'a' + "hello" + 'b'"#;
        let mut lexer = Lexer::new(input);

        // 'a' (rune)
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Some(TokenKind::RuneLiteral));

        // +
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::Operator(Operator::Plus)));

        // "hello" (string)
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Some(TokenKind::StringLiteral));

        // +
        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Some(TokenKind::Operator(Operator::Plus)));

        // 'b' (rune)
        let token5 = lexer.next_token();
        assert_eq!(token5.kind, Some(TokenKind::RuneLiteral));

        // EOF
        let token6 = lexer.next_token();
        assert_eq!(token6.kind, Some(TokenKind::EOF));

        // No errors
        assert_eq!(lexer.errors.len(), 0);
    }

    #[test]
    fn struct_start() {
        let input = "struct";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Struct)));
    }

    #[test]
    fn switch_start() {
        let input = "switch";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Switch)));
    }

    #[test]
    fn type_start() {
        let input = "type";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Type)));
    }

    #[test]
    fn var_start() {
        let input = "var";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Keyword(Keyword::Var)));
    }

    #[test]
    fn plus_start() {
        let input = "+";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Plus)));
    }

    #[test]
    fn minus_start() {
        let input = "-";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Minus)));
    }

    #[test]
    fn star_start() {
        let input = "*";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Star)));
    }

    #[test]
    fn slash_start() {
        let input = "/";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Slash)));
    }

    #[test]
    fn percent_start() {
        let input = "%";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Percent)));
    }

    #[test]
    fn ampersand_start() {
        let input = "&";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Ampersand)));
    }

    #[test]
    fn pipe_start() {
        let input = "|";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Pipe)));
    }

    #[test]
    fn caret_start() {
        let input = "^";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Caret)));
    }

    #[test]
    fn less_less_start() {
        let input = "<<";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::LessLess)));
    }

    #[test]
    fn greater_greater_start() {
        let input = ">>";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(
            token.kind,
            Some(TokenKind::Operator(Operator::GreaterGreater))
        );
    }

    #[test]
    fn ampersand_caret_start() {
        let input = "&^";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(
            token.kind,
            Some(TokenKind::Operator(Operator::AmpersandCaret))
        );
    }

    #[test]
    fn plus_equal_start() {
        let input = "+=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::PlusEqual));
    }

    #[test]
    fn minus_equal_start() {
        let input = "-=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::MinusEqual));
    }

    #[test]
    fn star_equal_start() {
        let input = "*=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::StarEqual));
    }

    #[test]
    fn slash_equal_start() {
        let input = "/=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::SlashEqual));
    }

    #[test]
    fn percent_equal_start() {
        let input = "%=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::PercentEqual));
    }

    #[test]
    fn ampersand_equal_start() {
        let input = "&=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::AmpersandEqual));
    }

    #[test]
    fn pipe_equal_start() {
        let input = "|=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::PipeEqual));
    }

    #[test]
    fn caret_equal_start() {
        let input = "^=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::CaretEqual));
    }

    #[test]
    fn less_less_equal_start() {
        let input = "<<=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LessLessEqual));
    }

    #[test]
    fn greater_greater_equal_start() {
        let input = ">>=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::GreaterGreaterEqual));
    }

    #[test]
    fn ampersand_caret_equal_start() {
        let input = "&^=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::AmpersandCaretEqual));
    }

    #[test]
    fn and_and_start() {
        let input = "&&";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::AndAnd)));
    }

    #[test]
    fn pipe_pipe_start() {
        let input = "||";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::PipePipe)));
    }

    #[test]
    fn less_minus_start() {
        let input = "<-";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LessMinus));
    }

    #[test]
    fn plus_plus_start() {
        let input = "++";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::PlusPlus));
    }

    #[test]
    fn minus_minus_start() {
        let input = "--";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::MinusMinus));
    }

    #[test]
    fn equal_equal_start() {
        let input = "==";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::EqualEqual)));
    }

    #[test]
    fn bang_equal_start() {
        let input = "!=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::BangEqual)));
    }

    #[test]
    fn less_start() {
        let input = "<";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Less)));
    }

    #[test]
    fn less_equal_start() {
        let input = "<=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::LessEqual)));
    }

    #[test]
    fn greater_start() {
        let input = ">";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Operator(Operator::Greater)));
    }

    #[test]
    fn greater_equal_start() {
        let input = ">=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(
            token.kind,
            Some(TokenKind::Operator(Operator::GreaterEqual))
        );
    }

    #[test]
    fn equal_start() {
        let input = "=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Equal));
    }

    #[test]
    fn colon_equal_start() {
        let input = ":=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::ColonEqual));
    }

    #[test]
    fn bang_start() {
        let input = "!";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Bang));
    }

    #[test]
    fn dot_dot_dot_start() {
        let input = "...";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::DotDotDot));
    }

    #[test]
    fn dot_start() {
        let input = ".";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Dot));
    }

    #[test]
    fn colon_start() {
        let input = ":";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Colon));
    }

    #[test]
    fn comma_start() {
        let input = ",";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Comma));
    }

    #[test]
    fn semicolon_start() {
        let input = ";";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Semicolon));
    }

    #[test]
    fn left_bracket_start() {
        let input = "[";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LeftBracket));
    }

    #[test]
    fn right_bracket_start() {
        let input = "]";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::RightBracket));
    }

    #[test]
    fn left_brace_start() {
        let input = "{";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LeftBrace));
    }

    #[test]
    fn right_brace_start() {
        let input = "}";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::RightBrace));
    }

    #[test]
    fn eof_start() {
        let input = "";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::EOF));
    }
}
