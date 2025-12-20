//! Error types for HTMS compiler

use crate::Location;
use thiserror::Error;

/// Lexer error
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct LexerError {
    pub message: String,
    pub location: Location,
}

/// Parser error
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct ParseError {
    pub message: String,
    pub location: Location,
}

/// Semantic analysis error
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct SemanticError {
    pub message: String,
    pub location: Location,
}

impl LexerError {
    pub fn new(message: impl Into<String>, location: Location) -> Self {
        Self {
            message: message.into(),
            location,
        }
    }
}

impl ParseError {
    pub fn new(message: impl Into<String>, location: Location) -> Self {
        Self {
            message: message.into(),
            location,
        }
    }
}

impl SemanticError {
    pub fn new(message: impl Into<String>, location: Location) -> Self {
        Self {
            message: message.into(),
            location,
        }
    }
}
