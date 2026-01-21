#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // Utilities
    SingleLineComment,
    StartBlockComment,
    EndBlockComment,
    EOF,

    // Keywords
    Break,
    Case,
    Chan,
    Const,
    Continue,
    Default,
    Defer,
    Else,
    Fallthrough,
    For,
    Func,
    Go,
    Goto,
    If,
    Import,
    Interface,
    Map,
    Package,
    Range,
    Return,
    Select,
    Struct,
    Switch,
    Type,
    Var,

    // Operators and Punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    LessLess,
    GreaterGreater,
    AmpersandCaret,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AmpersandEqual,
    PipeEqual,
    CaretEqual,
    LessLessEqual,
    GreaterGreaterEqual,
    AmpersandCaretEqual,
    AndAnd,
    PipePipe,
    LessMinus,
    PlusPlus,
    MinusMinus,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    ColonEqual,
    Bang,
    DotDotDot,
    Dot,
    Colon,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    // Literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    // ImaginaryLiteral,
    RuneLiteral,
    StringLiteral,
}

fn is_valid_string_content(content: &str) -> bool {
    let mut chars = content.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            // Escape sequence - consume the next character
            if chars.next().is_none() {
                // Incomplete escape sequence at end
                return false;
            }
            // TODO: Validate specific escape sequences here,
            // Currently we just check that backslashes are followed by another character
        }
        // All other characters are valid inside strings
    }
    true
}

fn is_valid_rune_content(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let mut chars = value.chars();
    let first_char = chars.next().unwrap();

    if first_char == '\\' {
        // Escape sequence - must have exactly one more character
        chars.next().is_some() && chars.next().is_none()
    } else {
        // Single character rune - must be exactly one character
        chars.next().is_none()
    }
}

impl TokenKind {
    pub fn from_str(value: &str) -> Option<TokenKind> {
        if value.is_empty() || value.chars().all(|c| c.is_whitespace()) {
            return None;
        }

        let is_integer_literal = value.chars().all(|c| c.is_ascii_digit());

        let is_float_literal = value.contains('.')
            && value.chars().all(|c| c.is_ascii_digit() || c == '.')
            && value.matches('.').count() == 1
            && !value.starts_with('.')
            && !value.ends_with('.');

        // Check for valid identifier (starts with letter or underscore, followed by alphanumeric or underscore)
        let is_valid_identifier = if let Some(first_char) = value.chars().next() {
            (first_char.is_ascii_alphabetic() || first_char == '_')
                && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        } else {
            false
        };

        // Check for valid string literal (starts and ends with quotes)
        let is_valid_string = value.len() >= 2
            && value.starts_with('"')
            && value.ends_with('"')
            && is_valid_string_content(&value[1..value.len() - 1]);

        // Check for valid rune literal (starts and ends with single quotes)
        let is_valid_rune = value.len() >= 3  // Minimum: 'a' 
            && value.starts_with('\'')
            && value.ends_with('\'')
            && is_valid_rune_content(&value[1..value.len()-1]);

        match value {
            // Keywords
            "break" => Some(TokenKind::Break),
            "case" => Some(TokenKind::Case),
            "chan" => Some(TokenKind::Chan),
            "const" => Some(TokenKind::Const),
            "continue" => Some(TokenKind::Continue),
            "default" => Some(TokenKind::Default),
            "defer" => Some(TokenKind::Defer),
            "else" => Some(TokenKind::Else),
            "fallthrough" => Some(TokenKind::Fallthrough),
            "for" => Some(TokenKind::For),
            "func" => Some(TokenKind::Func),
            "go" => Some(TokenKind::Go),
            "goto" => Some(TokenKind::Goto),
            "if" => Some(TokenKind::If),
            "import" => Some(TokenKind::Import),
            "interface" => Some(TokenKind::Interface),
            "map" => Some(TokenKind::Map),
            "package" => Some(TokenKind::Package),
            "range" => Some(TokenKind::Range),
            "return" => Some(TokenKind::Return),
            "select" => Some(TokenKind::Select),
            "struct" => Some(TokenKind::Struct),
            "switch" => Some(TokenKind::Switch),
            "type" => Some(TokenKind::Type),
            "var" => Some(TokenKind::Var),
            // Operators and Punctuation
            "..." => Some(TokenKind::DotDotDot),
            "<<=" => Some(TokenKind::LessLessEqual),
            ">>=" => Some(TokenKind::GreaterGreaterEqual),
            "&^=" => Some(TokenKind::AmpersandCaretEqual),
            "+=" => Some(TokenKind::PlusEqual),
            "-=" => Some(TokenKind::MinusEqual),
            "*=" => Some(TokenKind::StarEqual),
            "/=" => Some(TokenKind::SlashEqual),
            "%=" => Some(TokenKind::PercentEqual),
            "&=" => Some(TokenKind::AmpersandEqual),
            "|=" => Some(TokenKind::PipeEqual),
            "^=" => Some(TokenKind::CaretEqual),
            "&&" => Some(TokenKind::AndAnd),
            "||" => Some(TokenKind::PipePipe),
            "<-" => Some(TokenKind::LessMinus),
            "++" => Some(TokenKind::PlusPlus),
            "--" => Some(TokenKind::MinusMinus),
            "==" => Some(TokenKind::EqualEqual),
            "!=" => Some(TokenKind::BangEqual),
            "<" => Some(TokenKind::Less),
            "<=" => Some(TokenKind::LessEqual),
            ">" => Some(TokenKind::Greater),
            ">=" => Some(TokenKind::GreaterEqual),
            ":=" => Some(TokenKind::ColonEqual),
            "<<" => Some(TokenKind::LessLess),
            ">>" => Some(TokenKind::GreaterGreater),
            "&^" => Some(TokenKind::AmpersandCaret),
            "!" => Some(TokenKind::Bang),
            "=" => Some(TokenKind::Equal),
            "+" => Some(TokenKind::Plus),
            "-" => Some(TokenKind::Minus),
            "*" => Some(TokenKind::Star),
            "/" => Some(TokenKind::Slash),
            "%" => Some(TokenKind::Percent),
            "&" => Some(TokenKind::Ampersand),
            "|" => Some(TokenKind::Pipe),
            "^" => Some(TokenKind::Caret),
            "." => Some(TokenKind::Dot),
            ":" => Some(TokenKind::Colon),
            "," => Some(TokenKind::Comma),
            ";" => Some(TokenKind::Semicolon),
            "(" => Some(TokenKind::LeftParen),
            ")" => Some(TokenKind::RightParen),
            "[" => Some(TokenKind::LeftBracket),
            "]" => Some(TokenKind::RightBracket),
            "{" => Some(TokenKind::LeftBrace),
            "}" => Some(TokenKind::RightBrace),
            _ => {
                if is_integer_literal {
                    Some(TokenKind::IntegerLiteral)
                } else if is_float_literal {
                    Some(TokenKind::FloatLiteral)
                } else if is_valid_identifier {
                    Some(TokenKind::Identifier)
                } else if is_valid_string {
                    Some(TokenKind::StringLiteral)
                } else if is_valid_rune {
                    Some(TokenKind::RuneLiteral)
                } else {
                    // Invalid token - not a keyword, operator, literal, or valid identifier
                    None
                }
            }
        }
    }

    pub fn is_tokenizeable(value: &str) -> bool {
        TokenKind::from_str(value).is_some()
    }

    pub fn could_match(input: &str) -> bool {
        const TOKENS: &[&str] = &[
            // Keywords
            "break",
            "case",
            "chan",
            "const",
            "continue",
            "default",
            "defer",
            "else",
            "fallthrough",
            "for",
            "func",
            "go",
            "goto",
            "if",
            "import",
            "interface",
            "map",
            "package",
            "range",
            "return",
            "select",
            "struct",
            "switch",
            "type",
            "var",
            // Operators and Punctuation
            "+",
            "-",
            "*",
            "/",
            "%",
            "&",
            "|",
            "^",
            "<<",
            ">>",
            "&^",
            "+=",
            "-=",
            "*=",
            "/=",
            "%=",
            "&=",
            "|=",
            "^=",
            "<<=",
            ">>=",
            "&^=",
            "&&",
            "||",
            "<-",
            "++",
            "--",
            "==",
            "!=",
            "<",
            "<=",
            ">",
            ">=",
            "=",
            ":=",
            "!",
            "...",
            ".",
            ":",
            ",",
            ";",
            "(",
            ")",
            "[",
            "]",
            "{",
            "}",
        ];

        // Check if input could be a prefix of any static token
        if TOKENS.iter().any(|&t| t.starts_with(input)) {
            return true;
        }

        // Check if input could be a partial identifier (letters, digits, underscore, but must start with letter or underscore)
        if let Some(first_char) = input.chars().next() {
            if (first_char.is_ascii_alphabetic() || first_char == '_')
                && input.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                return true;
            }
        }

        // Check if input could be a partial integer literal (only digits)
        if !input.is_empty() && input.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }

        // Check if input could be a partial float literal (digits with at most one dot, not starting with dot)
        if !input.is_empty() && !input.starts_with('.') {
            let dot_count = input.matches('.').count();
            if dot_count <= 1 && input.chars().all(|c| c.is_ascii_digit() || c == '.') {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_tokenizes() {
        let token = TokenKind::from_str("func");
        assert_eq!(token, Some(TokenKind::Func));
    }

    #[test]
    fn identifier_tokenizes() {
        assert_eq!(TokenKind::from_str("main"), Some(TokenKind::Identifier));
        assert_eq!(
            TokenKind::from_str("variable123"),
            Some(TokenKind::Identifier)
        );
        assert_eq!(
            TokenKind::from_str("_underscore"),
            Some(TokenKind::Identifier)
        );
    }

    #[test]
    fn integer_literal_tokenizes() {
        assert_eq!(TokenKind::from_str("123"), Some(TokenKind::IntegerLiteral));
        assert_eq!(TokenKind::from_str("0"), Some(TokenKind::IntegerLiteral));
    }

    #[test]
    fn float_literal_tokenizes() {
        assert_eq!(
            TokenKind::from_str("123.456"),
            Some(TokenKind::FloatLiteral)
        );
        assert_eq!(TokenKind::from_str("0.5"), Some(TokenKind::FloatLiteral));
    }

    #[test]
    fn strings_tokenize() {
        assert_eq!(
            TokenKind::from_str("\"hello\""),
            Some(TokenKind::StringLiteral)
        );
        assert_eq!(
            TokenKind::from_str("\"hello world\""),
            Some(TokenKind::StringLiteral)
        );
        assert_eq!(
            TokenKind::from_str("\"hello \\\"quoted\\\" text\""),
            Some(TokenKind::StringLiteral)
        );
    }

    #[test]
    fn rune_literal_tokenizes() {
        assert_eq!(TokenKind::from_str("'a'"), Some(TokenKind::RuneLiteral));
        assert_eq!(TokenKind::from_str("'\\n'"), Some(TokenKind::RuneLiteral));
    }

    #[test]
    fn invalid_tokens_return_none() {
        assert_eq!(TokenKind::from_str("123."), None); // ends with dot
        assert_eq!(TokenKind::from_str(".123"), None); // starts with dot
        assert_eq!(TokenKind::from_str("12.3.4"), None); // multiple dots
        assert_eq!(TokenKind::from_str("main()"), None); // contains symbol
        assert_eq!(TokenKind::from_str("123abc"), None); // mixed number and letter
    }
}
