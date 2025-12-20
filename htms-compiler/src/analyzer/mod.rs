//! Semantic analyzer for HTMS
//!
//! Validates the AST and builds a symbol table.

mod symbols;
mod resolver;

pub use symbols::{Symbol, SymbolKind, SymbolTable};
pub use resolver::analyze;
