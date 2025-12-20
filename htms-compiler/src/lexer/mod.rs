//! Lexer (tokenizer) for HTMS
//!
//! Converts source code into a stream of tokens.

mod tokens;
mod scanner;

pub use tokens::{Token, TokenKind};
pub use scanner::tokenize;
