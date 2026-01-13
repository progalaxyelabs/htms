# CLAUDE.md - htms-compiler Developer Guide

## Quick Overview

**htms-compiler** is a Rust/WASM-based compiler for HTMS (HTM Script), a declarative UI language that compiles to type-safe TypeScript.

**Project Type:** Standalone library (Rust + Node.js/WASM)
**Tech Stack:** Rust (lexer, parser, analyzer, codegen) → WebAssembly → Node.js
**Main Output:** TypeScript code (templates.ts, router.ts, events.ts)

## Project Structure

```
htms-compiler/
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── lexer/           # Tokenization (Logos-based)
│   ├── parser/          # Recursive descent parser → AST
│   ├── analyzer/        # Semantic analysis & symbol table
│   └── codegen/         # TypeScript code generation
├── tests/               # Integration tests
├── examples/            # Sample HTMS programs
├── Cargo.toml          # Rust dependencies
├── package.json        # Node.js/npm metadata
├── tsconfig.json       # TypeScript config for types
└── build-wasm.sh       # WASM build script
```

## Key Build Commands

```bash
# Build WASM for Node.js
npm run build

# Release build (optimized)
npm run build:release

# Build for browser/web
npm run build:web

# Run tests
npm test

# Clean artifacts
npm run clean

# Publish to npm
npm run publish:wasm
```

## Development Workflow

1. **Make Changes:** Edit Rust source in `src/`
2. **Build:** `npm run build` (generates WASM in `pkg/`)
3. **Test:** `npm test` (cargo tests)
4. **Check Generated Code:** Review output in `dist/` after compilation

## Key APIs

### Main Export: `compile(source, options)`

```rust
// src/lib.rs
pub fn compile(source: String, options: Option<CompileOptions>) -> CompileResult
```

**CompileOptions:**
- `generate_router: bool` - Include router.ts (default: true)
- `generate_events: bool` - Include events.ts (default: true)

**CompileResult:**
```typescript
{
  success: boolean,
  files: Array<{path: string, content: string}>,
  diagnostics: Array<{severity: string, message: string, location: {line, column}}>
}
```

## Language Features

- **Components:** Reusable UI elements with typed parameters
- **Conditionals:** @if/@else for branching
- **Loops:** @each for list rendering
- **Events:** onClick, onChange, etc.
- **Routing:** page declarations for SPA routes
- **Slots:** Named content insertion points
- **Type Safety:** Parameter types validated at compile time

## Common Tasks

### Adding a New Language Feature

1. Add token to `src/lexer/tokens.rs`
2. Update parser in `src/parser/` to handle token
3. Update AST in `src/ast.rs`
4. Update analyzer in `src/analyzer/` for semantic checks
5. Update codegen in `src/codegen/` to generate TypeScript
6. Add tests in `tests/`

### Debugging Compilation

Check `result.diagnostics` array:
- Each diagnostic has `severity` (error/warning), `message`, `location`
- Location includes `line` and `column` (0-indexed)

## Dependencies

**Rust Crates:**
- `logos` 0.14 - Lexer generation (fast)
- `regex` 1.10 - String interpolation
- `serde/serde_json` - Serialization for WASM boundary
- `thiserror` - Error types
- `wasm-bindgen` - JS ↔ Rust glue

**Node.js:**
- TypeScript 5.3+ (for type checking generated code)
- vitest - Testing framework

## Build Output

When `npm run build:release` runs:
1. Rust code compiled to WASM
2. Generated in `pkg/` directory:
   - `htms_compiler.wasm` (binary)
   - `htms_compiler.js` (JS wrapper)
   - `htms_compiler.d.ts` (TypeScript types)
   - `package.json` (for npm publish)

## Testing Strategy

- **Unit tests:** In Rust files (`#[test]`)
- **Integration tests:** In `tests/` directory
- **Examples:** In `examples/` - runnable HTMS programs

Run all: `npm test`

## Known Limitations

- No CSS-in-JS yet (stylesheets handled separately)
- Single-file output only
- Limited optimization of generated code
- No external library imports

## Publishing

```bash
npm run publish:wasm
# Publishes @progalaxyelabs/htms-compiler to npm
# Requires npm credentials and access to org
```

## Useful Links

- Repository: https://github.com/progalaxyelabs/htms
- Issues: https://github.com/progalaxyelabs/htms/issues
- Related: htms-cli, htms-runtime, htms-vscode

## Version Info

- **Cargo Version:** 0.5.1 (Rust package)
- **npm Version:** 0.1.0 (Node.js package)
- **Node.js:** >=18.0.0 required
- **Edition:** Rust 2021
