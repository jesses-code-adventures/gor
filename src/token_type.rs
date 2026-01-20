#[derive(Debug, PartialEq)]
pub enum TokenType {
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

    //
}

impl TokenType {
    pub fn from_str(value: &str) -> Option<TokenType> {
        match value {
            "break" => Some(TokenType::Break),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Break => "break",
            _ => "",
        }
    }

    pub fn is_tokenizeable(value: &str) -> bool {
        match TokenType::from_str(value) {
            Some(_) => true,
            None => false,
        }
    }
}
