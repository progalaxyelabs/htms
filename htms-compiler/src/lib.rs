//! HTMS Compiler
//!
//! A compiler for HTMS (HTM Script) - a declarative language for building web UIs.
//!
//! # Example
//!
//! ```rust
//! use htms_compiler::compile;
//!
//! let source = "component NavBar { nav [class: \"navbar\"] { } }";
//! let result = compile(source);
//! assert!(result.files.len() > 0);
//! ```

pub mod lexer;
pub mod parser;
pub mod analyzer;
pub mod codegen;
pub mod error;
pub mod ast;

use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// A generated output file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// Relative path (e.g., "components/nav-bar.hbs")
    pub path: String,
    /// File content
    pub content: String,
}

/// Diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Source location
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

/// A diagnostic message (error, warning, or info)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// Generated files
    pub files: Vec<GeneratedFile>,
    /// Diagnostics (errors, warnings)
    pub diagnostics: Vec<Diagnostic>,
    /// Whether compilation succeeded (no errors)
    pub success: bool,
}

/// Compile options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOptions {
    /// Generate router.ts
    #[serde(default = "default_true")]
    pub generate_router: bool,
    /// Generate events.ts
    #[serde(default = "default_true")]
    pub generate_events: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            generate_router: true,
            generate_events: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Compile HTMS source code to HBS templates and TypeScript
pub fn compile(source: &str) -> CompileResult {
    compile_with_options(source, &CompileOptions::default())
}

/// Compile HTMS source code with options
pub fn compile_with_options(source: &str, options: &CompileOptions) -> CompileResult {
    let mut diagnostics = Vec::new();

    // Phase 1: Lexing
    let tokens = match lexer::tokenize(source) {
        Ok(tokens) => tokens,
        Err(errors) => {
            for err in errors {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: err.message,
                    location: err.location,
                    code: Some("E001".to_string()),
                });
            }
            return CompileResult {
                files: vec![],
                diagnostics,
                success: false,
            };
        }
    };

    // Phase 2: Parsing
    let ast = match parser::parse(&tokens) {
        Ok(ast) => ast,
        Err(errors) => {
            for err in errors {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: err.message,
                    location: err.location,
                    code: Some("E002".to_string()),
                });
            }
            return CompileResult {
                files: vec![],
                diagnostics,
                success: false,
            };
        }
    };

    // Phase 3: Analysis
    let (symbols, analysis_diagnostics) = analyzer::analyze(&ast);
    diagnostics.extend(analysis_diagnostics);

    // Check for errors
    let has_errors = diagnostics.iter().any(|d| d.severity == Severity::Error);
    if has_errors {
        return CompileResult {
            files: vec![],
            diagnostics,
            success: false,
        };
    }

    // Phase 4: Code Generation
    let files = codegen::generate(&ast, &symbols, options);

    CompileResult {
        files,
        diagnostics,
        success: true,
    }
}

// ============================================================================
// WASM Bindings
// ============================================================================

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn compile_wasm(source: &str) -> JsValue {
    let result = compile(source);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn compile_with_options_wasm(source: &str, options: JsValue) -> JsValue {
    let options: CompileOptions = serde_wasm_bindgen::from_value(options).unwrap_or_default();
    let result = compile_with_options(source, &options);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn check_wasm(source: &str) -> JsValue {
    // Parse and analyze only, no code generation
    let mut diagnostics = Vec::new();

    let tokens = match lexer::tokenize(source) {
        Ok(tokens) => tokens,
        Err(errors) => {
            for err in errors {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    message: err.message,
                    location: err.location,
                    code: Some("E001".to_string()),
                });
            }
            return serde_wasm_bindgen::to_value(&diagnostics).unwrap();
        }
    };

    if let Ok(ast) = parser::parse(&tokens) {
        let (_, analysis_diagnostics) = analyzer::analyze(&ast);
        diagnostics.extend(analysis_diagnostics);
    }

    serde_wasm_bindgen::to_value(&diagnostics).unwrap()
}
