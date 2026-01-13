# Changelog

All notable changes to HTMS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Documentation
- Added comprehensive documentation audit report (January 2026)
- Updated HLD.md to reflect current version (0.5.1) and production status
- Created CHANGELOG.md to track version history

## [0.5.1] - 2025-12-26

### Added
- Full DOM API generation (pure `document.createElement()` calls)
- VSCode extension with syntax highlighting and diagnostics
- Comprehensive documentation suite (5,000+ lines)
- AI agent guide (AI-AGENT-GUIDE.md) for automated development
- Support for both TypeScript and HTML output formats
- Complete examples: Todo App and Instagram Clone
- Vite plugin for seamless integration
- Two-way data binding with `bind:` attribute
- Event modifiers (`.prevent`, `.stop`, `.once`)
- Element directives (`@if`, `@for`)
- Route parameter support in pages

### Changed
- Compiler now generates pure DOM API calls instead of Handlebars templates
- Improved error messages and diagnostics with line/column tracking
- Enhanced CLI with watch mode and better output formatting
- Optimized WASM compilation for better performance

### Fixed
- XSS vulnerabilities eliminated through DOM API approach
- Component reference validation at compile time
- Various parsing and code generation bugs

### Documentation
- Complete language reference guide
- Component patterns and best practices
- Build and deployment guides for all major platforms
- API reference for generated code
- Getting started tutorial
- Two complete example applications

## [0.1.0] - 2024

### Added
- Initial HTMS language design
- Rust-based compiler with WASM support
- Basic syntax support for components, sections, and pages
- Lexer and parser implementation
- AST generation
- Basic code generator
- CLI tool for compilation
- Core language features:
  - Components with parameters
  - Sections and pages
  - Control flow (`@if`, `@each`)
  - Event handling
  - Context binding (`ctx.*`)
  - Attribute syntax `[key: value]`
  - Children blocks `{ }`
  - Text interpolation `{{ }}`

### Documentation
- Initial HLD (High-Level Design Document)
- Basic README

---

## Version History

- **0.5.1** (Current) - Production release with full features
- **0.1.0** - Initial design and prototype

---

## Links

- [GitHub Repository](https://github.com/progalaxyelabs/htms)
- [npm Package (htms-cli)](https://www.npmjs.com/package/@progalaxyelabs/htms-cli)
- [npm Package (htms-compiler)](https://www.npmjs.com/package/@progalaxyelabs/htms-compiler)
- [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=progalaxyelabs.htms-vscode)
- [Documentation](./docs/README.md)
