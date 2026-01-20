# AGENTS.md - Developer Guidelines for Gor (Go Lexer in Rust)

## Build, Lint, and Test Commands

### Building and Checking
- `cargo check` - Fast compile check without generating binaries
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version

### Testing
- `cargo test` - Run all tests
- `cargo test <testname>` - Run single test by name substring (e.g., `cargo test func_start`)
- `cargo test --lib` - Test only library code (not binaries)
- `cargo test --doc` - Test documentation examples

### Linting and Formatting
- `cargo clippy` - Run Clippy linter for code quality
- `cargo fmt` - Format code with rustfmt
- `cargo fmt -- --check` - Check if code is properly formatted

### Development Workflow
```bash
# Quick check during development
cargo check

# Format and lint before committing
cargo fmt
cargo clippy

# Run tests and build
cargo test
cargo build
```

## Code Style Guidelines

### Error Handling Patterns

**Custom Error Types:**
- Create dedicated error structs with position information
- Implement `std::error::Error` and `std::fmt::Display` traits
- Use enum variants for different error kinds

```rust
#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub position: Position,
}

#[derive(Debug)]
pub enum LexerErrorKind {
    IncompleteToken(String),
    UnexpectedToken(String),
}
```

**Error Collection:**
- Collect errors in vectors to continue processing rather than failing fast
- Use `Result<Option<T>, E>` to distinguish between incomplete tokens and errors
- Return `Ok(None)` for incomplete tokens that may be completed with more input

### Token Processing Patterns

**Lexer State Management:**
- Track position with separate anchor and current_position indices
- Use anchor to mark start of current token being processed
- Reset anchor when transitioning between token types

**Character Classification:**
- Use `matches!()` macro for clean character matching
- Separate symbol and whitespace detection functions

```rust
fn is_symbol(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | /* ... */)
}

fn is_whitespace(c: char) -> bool {
    matches!(c, '\n' | '\t' | '\r' | ' ')
}
```

### Data Structures

**Position Tracking:**
- Use `Position` struct for line and column information
- Include both start and end column positions
- Line numbers start at 1, columns start at 0

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    line: usize,
    column_start: usize,
    column_end: usize,
}
```

**Token Representation:**
- Use `Option<TokenKind>` to handle unclassified tokens
- Derive `Debug, PartialEq, Copy, Clone` for structs used in comparisons

### Token Classification

**Token Matching Strategy:**
- Implement `from_str()` method for direct token mapping
- Use `could_match()` for partial token validation during lexing
- Handle multi-character operators by checking for longer matches first

**Token Constants:**
- Define string constants array for tokens that support partial matching
- Use static string slices for performance

### Implementation Notes

**Lexer Logic:**
- Process input character by character with peeking capabilities
- Handle whitespace implicitly by skipping it
- Maintain error list separately from token stream
- Use loops with `continue` for token processing state machine

**Testing Approach:**
- Extensive unit tests for each token type
- Test both individual tokens and complex sequences
- Verify error collection and position reporting
- Use descriptive test names matching token types (e.g., `func_start`, `plus_equal_start`)

### Module Organization

**File Structure:**
```
src/
├── lib.rs          # Module declarations
├── main.rs         # Binary entry point
├── lexer.rs        # Main lexer implementation
├── token.rs        # Token struct and creation
├── token_type.rs   # TokenKind enum and matching logic
├── position.rs     # Position tracking
└── errors.rs       # Error types and handling
```

**Module Exports:**
- Export all modules from `lib.rs` for library usage
- Keep implementation details private within modules

### Development Practices

**Debugging:**
- Use `println!()` for temporary debugging output during development
- Remove debug prints before committing (grep for `println!` in PRs)

**TODO Management:**
- Document known issues and future work in README.md
- Use code comments for implementation notes and edge cases

**Performance Considerations:**
- Prefer `&str` over `String` for input parameters when ownership not needed
- Use copy types where possible (`Copy, Clone` traits)
- Consider lifetime parameters for input references (currently using owned `String`)

### Code Review Checklist

- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `cargo clippy` clean
- [ ] `cargo fmt` applied
- [ ] Error types properly implemented with position tracking
- [ ] Token matching handles all cases in `from_str()` and `could_match()`
- [ ] Tests added for new token types
- [ ] No leftover debug prints
- [ ] Position tracking accurate for error reporting</content>
<parameter name="filePath">/Users/jessewilliams/Coding/personal/gor/AGENTS.md