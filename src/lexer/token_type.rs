#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
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
    AndAnd,
    PipePipe,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl Operator {
    // https://go.dev/ref/spec#Operator_precedence
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::PipePipe => 1,
            Operator::AndAnd => 2,
            Operator::EqualEqual
            | Operator::BangEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual => 3,
            Operator::Plus | Operator::Minus | Operator::Pipe | Operator::Caret => 4,
            Operator::Star
            | Operator::Slash
            | Operator::Percent
            | Operator::LessLess
            | Operator::GreaterGreater
            | Operator::Ampersand
            | Operator::AmpersandCaret => 5,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
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
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // Utilities
    SingleLineComment,
    StartBlockComment,
    EndBlockComment,
    EOF,
    BeforeStart,

    // Literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    // ImaginaryLiteral,
    RuneLiteral,
    StringLiteral,

    // Keywords
    Keyword(Keyword),

    // Operators and Punctuation
    Operator(Operator),

    // Channels
    LessMinus,

    MinusMinus,
    PlusPlus,

    // Assignment
    Equal,
    ColonEqual,
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
    Backtick,
    DollarSign,
    Newline,
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
            "break" => Some(TokenKind::Keyword(Keyword::Break)),
            "case" => Some(TokenKind::Keyword(Keyword::Case)),
            "chan" => Some(TokenKind::Keyword(Keyword::Chan)),
            "const" => Some(TokenKind::Keyword(Keyword::Const)),
            "continue" => Some(TokenKind::Keyword(Keyword::Continue)),
            "default" => Some(TokenKind::Keyword(Keyword::Default)),
            "defer" => Some(TokenKind::Keyword(Keyword::Defer)),
            "else" => Some(TokenKind::Keyword(Keyword::Else)),
            "fallthrough" => Some(TokenKind::Keyword(Keyword::Fallthrough)),
            "for" => Some(TokenKind::Keyword(Keyword::For)),
            "func" => Some(TokenKind::Keyword(Keyword::Func)),
            "go" => Some(TokenKind::Keyword(Keyword::Go)),
            "goto" => Some(TokenKind::Keyword(Keyword::Goto)),
            "if" => Some(TokenKind::Keyword(Keyword::If)),
            "import" => Some(TokenKind::Keyword(Keyword::Import)),
            "interface" => Some(TokenKind::Keyword(Keyword::Interface)),
            "map" => Some(TokenKind::Keyword(Keyword::Map)),
            "package" => Some(TokenKind::Keyword(Keyword::Package)),
            "range" => Some(TokenKind::Keyword(Keyword::Range)),
            "return" => Some(TokenKind::Keyword(Keyword::Return)),
            "select" => Some(TokenKind::Keyword(Keyword::Select)),
            "struct" => Some(TokenKind::Keyword(Keyword::Struct)),
            "switch" => Some(TokenKind::Keyword(Keyword::Switch)),
            "type" => Some(TokenKind::Keyword(Keyword::Type)),
            "var" => Some(TokenKind::Keyword(Keyword::Var)),
            "..." => Some(TokenKind::DotDotDot),
            "<<=" => Some(TokenKind::LessLessEqual),
            ">>=" => Some(TokenKind::GreaterGreaterEqual),
            "&^=" => Some(TokenKind::AmpersandCaretEqual),
            // Operators and Punctuation
            "==" => Some(TokenKind::Operator(Operator::EqualEqual)),
            "!=" => Some(TokenKind::Operator(Operator::BangEqual)),
            "&&" => Some(TokenKind::Operator(Operator::AndAnd)),
            "||" => Some(TokenKind::Operator(Operator::PipePipe)),
            "+=" => Some(TokenKind::PlusEqual),
            "-=" => Some(TokenKind::MinusEqual),
            "*=" => Some(TokenKind::StarEqual),
            "/=" => Some(TokenKind::SlashEqual),
            "%=" => Some(TokenKind::PercentEqual),
            "&=" => Some(TokenKind::AmpersandEqual),
            "|=" => Some(TokenKind::PipeEqual),
            "^=" => Some(TokenKind::CaretEqual),
            "<-" => Some(TokenKind::LessMinus),
            "++" => Some(TokenKind::PlusPlus),
            "--" => Some(TokenKind::MinusMinus),
            ":=" => Some(TokenKind::ColonEqual),
            "<" => Some(TokenKind::Operator(Operator::Less)),
            "<=" => Some(TokenKind::Operator(Operator::LessEqual)),
            ">" => Some(TokenKind::Operator(Operator::Greater)),
            ">=" => Some(TokenKind::Operator(Operator::GreaterEqual)),
            "<<" => Some(TokenKind::Operator(Operator::LessLess)),
            ">>" => Some(TokenKind::Operator(Operator::GreaterGreater)),
            "&^" => Some(TokenKind::Operator(Operator::AmpersandCaret)),
            "+" => Some(TokenKind::Operator(Operator::Plus)),
            "-" => Some(TokenKind::Operator(Operator::Minus)),
            "*" => Some(TokenKind::Operator(Operator::Star)),
            "/" => Some(TokenKind::Operator(Operator::Slash)),
            "%" => Some(TokenKind::Operator(Operator::Percent)),
            "&" => Some(TokenKind::Operator(Operator::Ampersand)),
            "|" => Some(TokenKind::Operator(Operator::Pipe)),
            "^" => Some(TokenKind::Operator(Operator::Caret)),
            "!" => Some(TokenKind::Bang),
            "=" => Some(TokenKind::Equal),
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
            "`" => Some(TokenKind::Backtick),
            "$" => Some(TokenKind::DollarSign),
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
        assert_eq!(token, Some(TokenKind::Keyword(Keyword::Func)));
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
