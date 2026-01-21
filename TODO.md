# Parser Implementation TODO

## Phase 1: Core Parser Infrastructure

### 1.1 Parser Structure Setup
- [x] Create `src/parser/errors.rs` with basic parser error types
  - `ParserError` struct with position and error kind
  - `ParserErrorKind` enum (UnexpectedToken, MissingToken, etc.)
  - Error recovery synchronization points
- [x] Implement core `Parser` struct in `src/parser/parser.rs`
  - Fields: lexer, current_token, peek_token, errors
  - `new(input: String)` constructor
  - `advance()` method to consume current token
  - `peek()` method for lookahead
  - `expect_token(kind)` helper for required tokens
- [x] Add basic token management and lookahead
  - Initialize parser with first two tokens
  - Handle EOF gracefully
  - Implement synchronization for error recovery

### 1.2 Basic Parsing Framework
- [x] Create placeholder AST types in `src/parser/ast.rs`
  - `Program` struct (top-level container)
  - Basic `Expr` and `Stmt` placeholders
  - Position tracking integration
- [x] Implement `parse()` method skeleton
  - Main parsing entry point
  - Error collection strategy
  - Return `Result<Program, Vec<ParserError>>`
- [x] Add basic tests for parser initialization and token consumption

## Phase 2: Expression Parsing (Pratt Parser)

### 2.1 AST Expression Nodes
- [ ] Design hybrid AST approach for expressions
  - `ExprKind` enum with variants (Identifier, Literal, Binary, etc.)
  - `Expr` wrapper struct with kind + position
  - Binary operation types enum
- [ ] Implement expression AST constructors
  - Helper methods for creating positioned expressions
  - Type safety for operator combinations

### 2.2 Pratt Parser Implementation
- [ ] Create `src/parser/expressions.rs`
- [ ] Implement operator precedence table
  - Define precedence levels for Go operators
  - Left/right associativity rules
- [ ] Build Pratt parser core
  - `parse_expression(min_precedence)` method
  - Prefix expression parsing (unary, identifiers, literals)
  - Infix expression parsing (binary operators)
- [ ] Add expression parsing tests
  - Simple expressions: `x`, `42`, `"hello"`
  - Binary operations: `a + b`, `x * y + z`
  - Parentheses: `(a + b) * c`
  - Precedence testing: `a + b * c` should parse as `a + (b * c)`

## Phase 3: Statement Parsing

### 3.1 AST Statement Nodes
- [ ] Design statement AST structures
  - `StmtKind` enum (VarDecl, Return, Expression, Block)
  - `Stmt` wrapper with kind + position
  - Statement-specific data structures
- [ ] Implement statement constructors and helpers

### 3.2 Basic Statement Types
- [ ] Create `src/parser/statements.rs`
- [ ] Implement expression statements
  - Parse expression + semicolon
  - Handle missing semicolons with error recovery
- [ ] Add return statement parsing
  - `return` keyword + optional expression
  - Semicolon handling
- [ ] Implement variable declarations
  - `var name = value` syntax
  - Type annotations: `var name type = value`
  - Multiple declarations: `var a, b int = 1, 2`

### 3.3 Block Statements
- [ ] Parse block statements `{ ... }`
  - Opening/closing brace matching
  - Statement sequence parsing
  - Nested block support
- [ ] Add comprehensive statement tests
  - Each statement type in isolation
  - Nested blocks and complex combinations
  - Error cases and recovery

## Phase 4: Function Parsing

### 4.1 Function AST Design
- [ ] Design function-related AST nodes
  - `FunctionDecl` struct with name, params, return type, body
  - `Parameter` struct with name and type
  - `Type` enum for Go type system basics
- [ ] Integrate functions into top-level Program AST

### 4.2 Function Declaration Parsing
- [ ] Create `src/parser/functions.rs`
- [ ] Implement parameter list parsing
  - `(name type, name type)` syntax
  - Empty parameter lists
  - Parameter validation
- [ ] Add return type parsing
  - Single return types: `func name() int`
  - Multiple return types: `func name() (int, error)`
  - No return type (void functions)
- [ ] Parse function body as block statement
- [ ] Complete function declaration integration

## Phase 5: Error Recovery & Robustness

### 5.1 Error Recovery Mechanisms
- [ ] Implement synchronization points
  - Statement boundaries (`;`, `}`)
  - Top-level declarations (`func`, `var`)
  - Expression boundaries
- [ ] Add panic mode recovery
  - Skip tokens until sync point
  - Continue parsing after errors
  - Collect multiple errors per parse

### 5.2 Error Quality Improvements
- [ ] Enhance error messages
  - Expected vs. actual token reporting
  - Context-aware error descriptions
  - Suggestions for common mistakes
- [ ] Add error position tracking
  - Precise error location reporting
  - Multi-line error spans
  - Error underlining support

## Phase 6: Testing & Polish

### 6.1 Comprehensive Test Suite
- [ ] Add integration tests in `src/parser/tests.rs`
  - Complete Go function parsing
  - Complex expression combinations
  - Error recovery scenarios
  - Edge cases and malformed input
- [ ] Performance testing
  - Large file parsing benchmarks
  - Memory usage analysis

### 6.2 Developer Experience
- [ ] Create AST pretty-printing
  - Debug output for AST visualization
  - Formatted AST display
- [ ] Add parser usage examples
  - Example programs in README
  - API usage documentation
- [ ] Integration with main binary
  - Command-line parser invocation
  - File parsing workflow

## Implementation Notes

### Starting Point
1. Begin with Phase 1.1 - focus on getting the basic Parser struct working
2. Use simple placeholder AST types initially
3. Add real AST design incrementally as needed

### Architecture Decisions Made
- **Parser owns lexer**: Parser controls token consumption
- **Hybrid AST**: Enums for node types + wrapper structs for metadata
- **Error recovery**: Collect multiple errors, don't fail fast
- **Scope**: Functions + expressions + basic statements

### Testing Strategy
- Add tests incrementally with each feature
- Test both success cases and error recovery
- Focus on Go language compliance
- Use existing lexer tests as foundation

### Dependencies
- Parser depends on `lexer` package for tokens
- Parser depends on `primitives` package for Position
- AST nodes will use Position for error reporting
