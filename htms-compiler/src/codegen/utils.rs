//! Shared utilities for code generation

use crate::ast::*;
use std::collections::HashSet;

/// Analysis results for event/binding usage
#[derive(Default)]
pub struct EventAnalysis {
    pub has_events: bool,
    pub has_bindings: bool,
    pub event_types: HashSet<String>,  // click, submit, input, etc.
}

/// Analyze program for events and bindings
pub fn analyze_events(program: &Program) -> EventAnalysis {
    let mut analysis = EventAnalysis::default();
    for decl in &program.body {
        let nodes = match decl {
            Declaration::Component(c) => &c.body,
            Declaration::Section(s) => &s.body,
            Declaration::Page(p) => &p.body,
        };
        analyze_nodes(nodes, &mut analysis);
    }
    analysis
}

fn analyze_nodes(nodes: &[Node], analysis: &mut EventAnalysis) {
    for node in nodes {
        match node {
            Node::Element(el) => {
                for attr in &el.attributes {
                    if attr.name.starts_with("on") {
                        analysis.has_events = true;
                        // Extract event type: onClick -> click
                        let event_name = attr.name
                            .strip_prefix("on")
                            .unwrap_or(&attr.name)
                            .split('.')
                            .next()
                            .unwrap_or("")
                            .to_lowercase();
                        if !event_name.is_empty() {
                            analysis.event_types.insert(event_name);
                        }
                    }
                    if attr.name == "bind" {
                        analysis.has_bindings = true;
                    }
                }
                analyze_nodes(&el.children, analysis);
            }
            Node::If(stmt) => {
                analyze_nodes(&stmt.consequent, analysis);
                if let Some(alt) = &stmt.alternate {
                    match alt {
                        Alternate::Block(nodes) => analyze_nodes(nodes, analysis),
                        Alternate::ElseIf(elif) => analyze_nodes(&elif.consequent, analysis),
                    }
                }
            }
            Node::Each(stmt) => {
                analyze_nodes(&stmt.body, analysis);
            }
            _ => {}
        }
    }
}

/// Check if a program has any event handlers
pub fn program_has_events(program: &Program) -> bool {
    analyze_events(program).has_events
}
