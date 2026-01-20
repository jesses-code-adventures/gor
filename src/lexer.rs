use crate::errors::{LexerError, LexerErrorKind};
use crate::position::Position;
use crate::token::Token;
use crate::token_type::TokenKind;

pub struct Lexer {
    input: String, // TODO: this should be a stream or a &str but i cbf to deal with lifetimes
    current_position: usize,
    anchor: usize,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            current_position: 0,
            anchor: 0,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            match self.next() {
                Some(ch) => match ch {
                    ch if is_whitespace(ch) => {
                        self.handle_whitespace();
                        continue;
                    }
                    ch if is_symbol(ch) => {
                        println!("Handling symbol: '{}'", ch);
                        // Check if we need to finalize a pending word first
                        let symbol_pos = self.current_position - 1;
                        if self.anchor < symbol_pos {
                            // Check what type of characters we have pending
                            let pending_value = &self.input[self.anchor..symbol_pos];

                            // Only separate if the pending characters are not symbols
                            // (i.e., we're transitioning from word to symbol, not symbol to symbol)
                            if !pending_value.chars().all(is_symbol) {
                                println!("Found pending word '{}' before symbol", pending_value);

                                // Move current_position back to the symbol so it will be reprocessed
                                self.current_position = symbol_pos;

                                // Create word token
                                let word_token = match TokenKind::from_str(pending_value) {
                                    Some(_) => Token::new(
                                        pending_value,
                                        Position::new(self.current_line(), self.anchor, symbol_pos),
                                    ),
                                    None => {
                                        self.errors.push(LexerError::new(
                                            LexerErrorKind::UnexpectedToken(
                                                pending_value.to_string(),
                                            ),
                                            Position::new(
                                                self.current_line(),
                                                self.anchor,
                                                symbol_pos,
                                            ),
                                        ));
                                        Token {
                                            kind: None,
                                            position: Position::new(
                                                self.current_line(),
                                                self.anchor,
                                                symbol_pos,
                                            ),
                                        }
                                    }
                                };

                                // Set anchor for the symbol
                                self.anchor = symbol_pos;
                                return word_token;
                            }
                        }

                        if let Some(token) = self.handle_symbol() {
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
                    return Token {
                        kind: Some(TokenKind::EOF),
                        position: self.current_token_position(),
                    };
                }
            }
        }
    }

    fn handle_word(&mut self) -> Option<Token> {
        let value = self.proposed_token(false);
        println!("Value: '{}'", value);
        match self.tokenize(value) {
            Ok(Some(token)) => match token.kind {
                None => return None,
                _ => return Some(token),
            },
            Ok(None) => {
                return None;
            }
            Err(error) => {
                self.errors.push(error);
                self.anchor = self.current_position;
                return Some(Token {
                    kind: None,
                    position: self.current_token_position(),
                });
            }
        }
    }

    fn handle_symbol(&mut self) -> Option<Token> {
        let value = self.proposed_token(false);
        println!(
            "handle_symbol processing: '{}' (anchor={}, current_pos={})",
            value, self.anchor, self.current_position
        );
        match self.tokenize(value) {
            Ok(Some(token)) => {
                // Token was successfully created, advance anchor
                self.anchor = self.current_position;
                println!("Symbol token created, advancing anchor to {}", self.anchor);
                Some(token)
            }
            Ok(None) => None,
            Err(error) => {
                self.errors.push(error);
                self.anchor = self.current_position;
                Some(Token {
                    kind: None,
                    position: self.current_token_position(),
                })
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

    fn peek_prev(&self) -> Option<char> {
        if self.current_position <= 0 {
            return None;
        }
        let c = self.input.chars().nth(self.current_position - 1);
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
                // Check if we can match a longer token
                if let Some(next_c) = self.peek() {
                    if !self.peek_is_whitespace() {
                        let longer = value.to_string() + &next_c.to_string();
                        if TokenKind::could_match(&longer) {
                            return Ok(None);
                        }
                    }
                }
                // NOTE: we know the token will have a Some kind with the current
                // implementation, because we have already checked that TokenKind parses
                // this should probably be checked at some point though, in case Token::new ever
                // returns None for another reason
                return Ok(Some(Token::new(value, self.current_token_position())));
            }
            None => {
                // handles potential incomplete tokens - we only want to error on the last char of
                // a set of characters that could be a token, so that we're not erroring on every
                // character in a string
                if !self.peek_is_whitespace() {
                    return Ok(None);
                }
                match TokenKind::could_match(value) && self.peek_is_whitespace() {
                    true => {
                        return Err(LexerError::new(
                            LexerErrorKind::IncompleteToken(value.to_string()),
                            self.current_token_position(),
                        ));
                    }
                    false => {
                        return Err(LexerError::new(
                            LexerErrorKind::UnexpectedToken(value.to_string()),
                            self.current_token_position(),
                        ));
                    }
                }
            }
        };
    }

    fn current_token_position(&self) -> Position {
        Position::new(self.current_line(), self.anchor, self.current_position)
    }

    fn handle_whitespace(&mut self) {
        // No need to advance current_position, it's already advanced by next()
        self.anchor = self.current_position;
    }

    fn proposed_token(&self, already_iterated: bool) -> &str {
        &self.input[self.anchor..self.current_position - if already_iterated { 1 } else { 0 }]
    }

    fn current_line(&self) -> usize {
        let end = self.current_position.min(self.input.len());
        self.input[0..end].split('\n').count()
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
    use crate::errors::LexerErrorKind;
    #[test]
    fn three_errors() {
        let input = "asdf asdf asdf";
        let mut lexer = Lexer::new(input);
        loop {
            match lexer.next_token().kind {
                Some(TokenKind::EOF) => break,
                _ => {}
            }
        }
        assert_eq!(lexer.errors.len(), 3);
    }

    #[test]
    fn character_cutting_bug_demonstration() {
        let input = "hello world test";
        let mut lexer = Lexer::new(input);

        // Capture all errors to see the actual tokens being parsed
        let mut captured_errors = Vec::new();
        loop {
            let token = lexer.next_token();
            if let Some(TokenKind::EOF) = token.kind {
                break;
            }
            if token.kind.is_none() {
                captured_errors.push(lexer.errors.last().unwrap().clone());
            }
        }

        // Each word should produce exactly one error with the full word
        assert_eq!(captured_errors.len(), 3);
        assert_eq!(
            captured_errors[0].kind,
            LexerErrorKind::UnexpectedToken("hello".to_string())
        );
        assert_eq!(
            captured_errors[1].kind,
            LexerErrorKind::UnexpectedToken("world".to_string())
        );
        assert_eq!(
            captured_errors[2].kind,
            LexerErrorKind::UnexpectedToken("test".to_string())
        );
    }

    #[test]
    fn token_separation_for_main_function() {
        let input = "main()";
        let mut lexer = Lexer::new(input);

        // Should tokenize as: main (identifier/error), (, )
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, None); // main should be an unrecognized identifier
        assert_eq!(lexer.errors.len(), 1);
        assert_eq!(
            lexer.errors[0].kind,
            LexerErrorKind::UnexpectedToken("main".to_string())
        );

        let token2 = lexer.next_token();
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
        assert_eq!(token1.kind, None);
        assert_eq!(lexer.errors.len(), 1);
        assert_eq!(
            lexer.errors[0].kind,
            LexerErrorKind::UnexpectedToken("hello".to_string())
        );

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Some(TokenKind::Plus));

        let token3 = lexer.next_token();
        assert_eq!(token3.kind, None);
        assert_eq!(lexer.errors.len(), 2);
        assert_eq!(
            lexer.errors[1].kind,
            LexerErrorKind::UnexpectedToken("world".to_string())
        );

        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Some(TokenKind::Minus));

        let token5 = lexer.next_token();
        assert_eq!(token5.kind, None);
        assert_eq!(lexer.errors.len(), 3);
        assert_eq!(
            lexer.errors[2].kind,
            LexerErrorKind::UnexpectedToken("test".to_string())
        );

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
        assert_eq!(token.kind, Some(TokenKind::Func));
        let token = lexer.next_token();
        assert_eq!(token.kind, None);
        assert_eq!(lexer.errors.len(), 1);
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
        assert_eq!(token.kind, Some(TokenKind::Func));
    }

    #[test]
    fn break_start() {
        let input = "break";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Break));
    }

    #[test]
    fn case_start() {
        let input = "case";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Case));
    }

    #[test]
    fn chan_start() {
        let input = "chan";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Chan));
    }

    #[test]
    fn const_start() {
        let input = "const";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Const));
    }

    #[test]
    fn continue_start() {
        let input = "continue";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Continue));
    }

    #[test]
    fn default_start() {
        let input = "default";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Default));
    }

    #[test]
    fn defer_start() {
        let input = "defer";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Defer));
    }

    #[test]
    fn else_start() {
        let input = "else";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Else));
    }

    #[test]
    fn fallthrough_start() {
        let input = "fallthrough";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Fallthrough));
    }

    #[test]
    fn for_start() {
        let input = "for";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::For));
    }

    #[test]
    fn go_start() {
        let input = "go";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Go));
    }

    #[test]
    fn goto_start() {
        let input = "goto";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Goto));
    }

    #[test]
    fn if_start() {
        let input = "if";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::If));
    }

    #[test]
    fn import_start() {
        let input = "import";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Import));
    }

    #[test]
    fn interface_start() {
        let input = "interface";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Interface));
    }

    #[test]
    fn map_start() {
        let input = "map";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Map));
    }

    #[test]
    fn package_start() {
        let input = "package";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Package));
    }

    #[test]
    fn range_start() {
        let input = "range";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Range));
    }

    #[test]
    fn return_start() {
        let input = "return";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Return));
    }

    #[test]
    fn select_start() {
        let input = "select";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Select));
    }

    #[test]
    fn struct_start() {
        let input = "struct";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Struct));
    }

    #[test]
    fn switch_start() {
        let input = "switch";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Switch));
    }

    #[test]
    fn type_start() {
        let input = "type";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Type));
    }

    #[test]
    fn var_start() {
        let input = "var";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Var));
    }

    #[test]
    fn plus_start() {
        let input = "+";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Plus));
    }

    #[test]
    fn minus_start() {
        let input = "-";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Minus));
    }

    #[test]
    fn star_start() {
        let input = "*";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Star));
    }

    #[test]
    fn slash_start() {
        let input = "/";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Slash));
    }

    #[test]
    fn percent_start() {
        let input = "%";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Percent));
    }

    #[test]
    fn ampersand_start() {
        let input = "&";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Ampersand));
    }

    #[test]
    fn pipe_start() {
        let input = "|";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Pipe));
    }

    #[test]
    fn caret_start() {
        let input = "^";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Caret));
    }

    #[test]
    fn less_less_start() {
        let input = "<<";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LessLess));
    }

    #[test]
    fn greater_greater_start() {
        let input = ">>";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::GreaterGreater));
    }

    #[test]
    fn ampersand_caret_start() {
        let input = "&^";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::AmpersandCaret));
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
        assert_eq!(token.kind, Some(TokenKind::AndAnd));
    }

    #[test]
    fn pipe_pipe_start() {
        let input = "||";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::PipePipe));
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
        assert_eq!(token.kind, Some(TokenKind::EqualEqual));
    }

    #[test]
    fn bang_equal_start() {
        let input = "!=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::BangEqual));
    }

    #[test]
    fn less_start() {
        let input = "<";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Less));
    }

    #[test]
    fn less_equal_start() {
        let input = "<=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LessEqual));
    }

    #[test]
    fn greater_start() {
        let input = ">";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Greater));
    }

    #[test]
    fn greater_equal_start() {
        let input = ">=";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::GreaterEqual));
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
    fn left_paren_start() {
        let input = "(";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::LeftParen));
    }

    #[test]
    fn right_paren_start() {
        let input = ")";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::RightParen));
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
