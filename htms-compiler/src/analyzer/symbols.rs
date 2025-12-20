//! Symbol table for HTMS

use std::collections::HashMap;
use crate::Location;

/// Symbol kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Component,
    Section,
    Page,
}

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub usages: Vec<Location>,
}

/// Symbol table for tracking declarations
#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare a new symbol
    pub fn declare(&mut self, name: String, kind: SymbolKind, location: Location) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            return Err(format!("Duplicate declaration: '{}' is already defined", name));
        }
        self.symbols.insert(name.clone(), Symbol {
            name,
            kind,
            location,
            usages: Vec::new(),
        });
        Ok(())
    }

    /// Look up a symbol
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Look up a symbol mutably
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.symbols.get_mut(name)
    }

    /// Check if a symbol exists
    pub fn has(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Add a usage to a symbol
    pub fn add_usage(&mut self, name: &str, location: Location) {
        if let Some(symbol) = self.symbols.get_mut(name) {
            symbol.usages.push(location);
        }
    }

    /// Get all symbols
    pub fn all(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }

    /// Get symbols by kind
    pub fn by_kind(&self, kind: SymbolKind) -> impl Iterator<Item = &Symbol> {
        self.symbols.values().filter(move |s| s.kind == kind)
    }

    /// Get all component names
    pub fn components(&self) -> Vec<&str> {
        self.by_kind(SymbolKind::Component)
            .map(|s| s.name.as_str())
            .collect()
    }

    /// Get all section names
    pub fn sections(&self) -> Vec<&str> {
        self.by_kind(SymbolKind::Section)
            .map(|s| s.name.as_str())
            .collect()
    }

    /// Get all page names
    pub fn pages(&self) -> Vec<&str> {
        self.by_kind(SymbolKind::Page)
            .map(|s| s.name.as_str())
            .collect()
    }
}
