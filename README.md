# gor

Rust implementation of a Go lexer, just a bit of fun

## TODO

- LHS of values are being cut off on second tokens and beyond (eg "asdf asdf asdf" is coing through as "asdf sdf sdf") - observable with `cargo test`
- Classification of tokens is required, because currently we just look for contiguous non whitespace chars, but something like `main()` needs to parse into a [None (Maybe Ident), LParen, RParen].
- When we switch from a string to symbol, we need to ensure the previous string is tokenized correctly - this is probably best done when attempting to tokenize the word, we should peek next on alphabetic chars and return the token if the next char is a symbol like we do with whitespace
- Strings and multiline comments will need to be handled by giving the lexer some state, something like `is_parsing_string` on the Lexer that is enabled when we hit a " char (that isn't preceded by a \ within an existing string) and disabled when we hit a subsequent ".
- Runes will need to be handled the same as strings, but with ' instead of "

