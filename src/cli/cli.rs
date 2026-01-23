use crate::lexer::lexer::Lexer;
use crate::lexer::token_type::TokenKind;
use std::fs;
use std::path::PathBuf;

pub struct CLI {
    pub args: Vec<String>,
}

impl CLI {
    pub fn new(args: Vec<String>) -> CLI {
        CLI { args }
    }

    pub fn execute(&self) {
        self.verify_base_command();
        let subcommand = &self.args[1];

        match subcommand.as_str() {
            "dump-tokens" => self.handle_dump_tokens(),
            _ => {
                eprintln!("Unknown command: {}", subcommand);
            }
        }
    }

    fn handle_dump_tokens(&self) {
        let filename = &self.args[2];
        if filename.is_empty() {
            eprintln!("Usage: gor dump-tokens <filename>");
            std::process::exit(1);
        }
        let content = self.read_go_file(filename);
        let mut lexer = Lexer::new(&content);
        loop {
            let token = lexer.next_token();
            if let Some(TokenKind::EOF) = token.kind {
                break;
            }
            println!(
                "{}:{} {} {:?} {}",
                token.position.line,
                token.position.column_start,
                token.position.column_end,
                token.kind.unwrap_or(TokenKind::BeforeStart),
                token.value.escape_debug()
            );
        }
        let errors = lexer.errors();
        for error in errors {
            eprintln!(
                "Error at {}:{}: {:?}",
                error.position.line, error.position.column_start, error.kind
            );
        }
        if !errors.is_empty() {
            std::process::exit(1);
        }
    }

    fn verify_base_command(&self) {
        if self.args.len() < 2 {
            eprintln!("Usage: gor <command>");
            std::process::exit(1);
        }

        if !&self.args[0].contains("gor") {
            eprintln!("Unknown command: {}", self.args[0]);
            std::process::exit(1);
        }
    }

    fn validate_go_file(&self, path: &PathBuf) {
        if path.extension().and_then(|ext| ext.to_str()) != Some("go") {
            eprintln!("Error: file must have a .go extension");
            std::process::exit(1);
        }

        if !path.exists() {
            eprintln!("Error: file '{:?}' does not exist", path.to_str());
            std::process::exit(1);
        }
    }

    fn read_go_file(&self, filename: &str) -> String {
        let path = PathBuf::from(filename);
        self.validate_go_file(&path);

        match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
    }
}
