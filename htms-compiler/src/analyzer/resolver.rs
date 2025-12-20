//! Reference resolver and validator

use crate::ast::*;
use crate::{Diagnostic, Location, Severity};
use super::symbols::{SymbolKind, SymbolTable};

/// Analyze the AST and return symbol table + diagnostics
pub fn analyze(program: &Program) -> (SymbolTable, Vec<Diagnostic>) {
    let mut analyzer = Analyzer::new();
    analyzer.analyze(program);
    (analyzer.symbols, analyzer.diagnostics)
}

struct Analyzer {
    symbols: SymbolTable,
    diagnostics: Vec<Diagnostic>,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            diagnostics: Vec::new(),
        }
    }

    fn analyze(&mut self, program: &Program) {
        // First pass: collect declarations
        self.collect_declarations(program);

        // Second pass: resolve references
        self.resolve_references(program);

        // Third pass: validate
        self.validate(program);
    }

    // =========================================================================
    // First pass: collect declarations
    // =========================================================================

    fn collect_declarations(&mut self, program: &Program) {
        for decl in &program.body {
            match decl {
                Declaration::Component(c) => {
                    if let Err(msg) = self.symbols.declare(
                        c.name.clone(),
                        SymbolKind::Component,
                        c.loc,
                    ) {
                        self.error(&msg, c.loc);
                    }
                }
                Declaration::Section(s) => {
                    if let Err(msg) = self.symbols.declare(
                        s.name.clone(),
                        SymbolKind::Section,
                        s.loc,
                    ) {
                        self.error(&msg, s.loc);
                    }
                }
                Declaration::Page(p) => {
                    if let Err(msg) = self.symbols.declare(
                        p.name.clone(),
                        SymbolKind::Page,
                        p.loc,
                    ) {
                        self.error(&msg, p.loc);
                    }
                }
            }
        }
    }

    // =========================================================================
    // Second pass: resolve references
    // =========================================================================

    fn resolve_references(&mut self, program: &Program) {
        for decl in &program.body {
            match decl {
                Declaration::Component(c) => self.resolve_nodes(&c.body),
                Declaration::Section(s) => self.resolve_nodes(&s.body),
                Declaration::Page(p) => self.resolve_nodes(&p.body),
            }
        }
    }

    fn resolve_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.resolve_node(node);
        }
    }

    fn resolve_node(&mut self, node: &Node) {
        match node {
            Node::Element(e) => {
                self.resolve_nodes(&e.children);
            }
            Node::ComponentRef(r) => {
                // Check if component exists
                if !self.symbols.has(&r.name) {
                    self.error(
                        &format!("Undefined component: '{}'", r.name),
                        r.loc,
                    );
                } else {
                    self.symbols.add_usage(&r.name, r.loc);
                }
                self.resolve_nodes(&r.children);
            }
            Node::If(stmt) => {
                self.resolve_nodes(&stmt.consequent);
                if let Some(alt) = &stmt.alternate {
                    match alt {
                        Alternate::Block(nodes) => self.resolve_nodes(nodes),
                        Alternate::ElseIf(elif) => {
                            self.resolve_nodes(&elif.consequent);
                            if let Some(a) = &elif.alternate {
                                match a {
                                    Alternate::Block(n) => self.resolve_nodes(n),
                                    Alternate::ElseIf(_) => {} // Recursive handled above
                                }
                            }
                        }
                    }
                }
            }
            Node::Each(stmt) => {
                self.resolve_nodes(&stmt.body);
            }
            Node::Text(_) | Node::Slot(_) => {}
        }
    }

    // =========================================================================
    // Third pass: validate
    // =========================================================================

    fn validate(&mut self, program: &Program) {
        // Check for duplicate routes
        let mut routes: std::collections::HashMap<String, Location> = std::collections::HashMap::new();
        for decl in &program.body {
            if let Declaration::Page(p) = decl {
                if let Some(_existing_loc) = routes.get(&p.route) {
                    self.error(
                        &format!("Duplicate route: '{}' is already defined", p.route),
                        p.loc,
                    );
                } else {
                    // Validate route format
                    if !p.route.starts_with('/') {
                        self.error(
                            &format!("Invalid route: '{}' must start with '/'", p.route),
                            p.loc,
                        );
                    }
                    routes.insert(p.route.clone(), p.loc);
                }
            }
        }

        // Warn about unused components
        let unused_components: Vec<_> = self.symbols.all()
            .filter(|symbol| symbol.kind == SymbolKind::Component && symbol.usages.is_empty())
            .map(|s| (s.name.clone(), s.location))
            .collect();

        for (name, location) in unused_components {
            self.warning(
                &format!("Component '{}' is declared but never used", name),
                location,
            );
        }

        // Warn if no pages defined
        if self.symbols.by_kind(SymbolKind::Page).count() == 0 {
            self.warning(
                "No pages defined - at least one page is recommended",
                program.loc,
            );
        }
    }

    // =========================================================================
    // Helpers
    // =========================================================================

    fn error(&mut self, message: &str, location: Location) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            message: message.to_string(),
            location,
            code: Some("E003".to_string()),
        });
    }

    fn warning(&mut self, message: &str, location: Location) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            message: message.to_string(),
            location,
            code: Some("W001".to_string()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn analyze_source(source: &str) -> (SymbolTable, Vec<Diagnostic>) {
        let tokens = tokenize(source).unwrap();
        let ast = parse(&tokens).unwrap();
        analyze(&ast)
    }

    #[test]
    fn test_collect_declarations() {
        let (symbols, _) = analyze_source(r#"
            component NavBar { }
            section Hero { }
            page home "/" { }
        "#);

        assert!(symbols.has("NavBar"));
        assert!(symbols.has("Hero"));
        assert!(symbols.has("home"));
    }

    #[test]
    fn test_undefined_component() {
        let (_, diagnostics) = analyze_source(r#"
            page home "/" { UndefinedComponent }
        "#);

        assert!(diagnostics.iter().any(|d|
            d.severity == Severity::Error &&
            d.message.contains("Undefined component")
        ));
    }

    #[test]
    fn test_duplicate_route() {
        let (_, diagnostics) = analyze_source(r#"
            page home "/" { }
            page landing "/" { }
        "#);

        assert!(diagnostics.iter().any(|d|
            d.severity == Severity::Error &&
            d.message.contains("Duplicate route")
        ));
    }

    #[test]
    fn test_invalid_route() {
        let (_, diagnostics) = analyze_source(r#"
            page home "home" { }
        "#);

        assert!(diagnostics.iter().any(|d|
            d.severity == Severity::Error &&
            d.message.contains("must start with")
        ));
    }

    #[test]
    fn test_unused_component_warning() {
        let (_, diagnostics) = analyze_source(r#"
            component Unused { }
            page home "/" { }
        "#);

        assert!(diagnostics.iter().any(|d|
            d.severity == Severity::Warning &&
            d.message.contains("never used")
        ));
    }
}
