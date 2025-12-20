//! Parser for HTMS
//!
//! Converts a stream of tokens into an AST.

mod grammar;

use crate::ast::Program;
use crate::error::ParseError;
use crate::lexer::Token;

/// Parse tokens into an AST
pub fn parse(tokens: &[Token]) -> Result<Program, Vec<ParseError>> {
    grammar::Parser::new(tokens).parse()
}
