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
                Some(c) => {
                    println!("Processing char: '{}'", c);
                    match c {
                        '\n' | '\t' | '\r' | ' ' => {
                            self.handle_whitespace();
                            continue;
                        }
                        _ => {
                            let value = self.proposed_token(false);
                            println!("Value: '{}'", value);
                            match self.tokenize(value) {
                                Ok(Some(token)) => {
                                    println!("Tokenize returned Some: {:?}", token);
                                    match token.kind {
                                        None => continue, // TODO: check if this is right
                                        _ => return token,
                                    }
                                }
                                Ok(None) => {
                                    println!("Tokenize returned None");
                                    continue;
                                }
                                Err(error) => {
                                    println!("Tokenize returned Err: {:?}", error);
                                    self.errors.push(error);
                                    self.anchor = self.current_position;
                                    continue;
                                }
                            }
                        }
                    }
                }
                None => {
                    println!("End of input, returning EOF");
                    return Token::new("EOF", self.current_token_position());
                }
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
            Some(' ') => true,
            Some('\n') => true,
            Some('\t') => true,
            Some('\r') => true,
            _ => false,
        }
    }

    /// returns Some(token) if the token is tokenized, None if it is incomplete, err if there is an
    /// incomplete or unexpected full token
    fn tokenize(&self, value: &str) -> Result<Option<Token>, LexerError> {
        match TokenKind::from_str(value) {
            Some(_) => {
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
        self.current_position += 1;
        self.anchor = self.current_position;
    }

    fn proposed_token(&self, already_iterated: bool) -> &str {
        &self.input[self.anchor..self.current_position - if already_iterated { 1 } else { 0 }]
    }

    fn current_line(&self) -> usize {
        self.input[0..self.current_position].split('\n').count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_start() {
        let input = "func";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        assert_eq!(token.kind, Some(TokenKind::Func));
    }
}
