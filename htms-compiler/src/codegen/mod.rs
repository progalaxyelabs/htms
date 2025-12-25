//! Code generation for HTMS
//!
//! Generates TypeScript template functions or HTML from the AST.
//! No Handlebars dependency - compiles directly to JS/TS/HTML.

mod templates;
mod router;
mod events;
mod utils;
pub mod html;

use crate::ast::Program;
use crate::analyzer::SymbolTable;
use crate::{CompileOptions, GeneratedFile, OutputFormat};

/// Generate output files from AST
pub fn generate(
    program: &Program,
    symbols: &SymbolTable,
    options: &CompileOptions,
) -> Vec<GeneratedFile> {
    match options.output_format {
        OutputFormat::Html => {
            // Generate static HTML files
            html::generate(program, options)
        }
        OutputFormat::Typescript => {
            // Generate TypeScript/JavaScript files
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
    }
}
