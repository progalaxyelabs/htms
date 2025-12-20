//! Code generation for HTMS
//!
//! Generates TypeScript template functions from the AST.
//! No Handlebars dependency - compiles directly to JS/TS.

mod templates;
mod router;
mod events;
mod utils;

use crate::ast::Program;
use crate::analyzer::SymbolTable;
use crate::{CompileOptions, GeneratedFile};

/// Generate output files from AST
pub fn generate(
    program: &Program,
    symbols: &SymbolTable,
    options: &CompileOptions,
) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    // Generate template functions (components, sections, pages)
    files.push(templates::generate(program, symbols));

    // Generate router
    if options.generate_router {
        files.push(router::generate(program, symbols));
    }

    // Generate events only if the program actually has events
    if options.generate_events && utils::program_has_events(program) {
        files.push(events::generate(program, symbols));
    }

    files
}
