# @progalaxyelabs/htms-compiler

HTMS (HTM Script) compiler - A declarative language for building web UIs with compile-time safety.

## Installation

```bash
npm install @progalaxyelabs/htms-compiler
```

## Usage

```javascript
import { compile } from '@progalaxyelabs/htms-compiler';

const source = `
component Button(text: string, onClick: function) {
  button [onClick: onClick] {
    {{ text }}
  }
}

page home "/" {
  Button(text: "Click Me", onClick: handleClick)
}
`;

const result = compile(source);

if (result.success) {
  console.log('Compiled successfully!');
  result.files.forEach(file => {
    console.log(`Generated: ${file.path}`);
    // file.content contains the TypeScript code
  });
} else {
  console.error('Compilation errors:');
  result.diagnostics.forEach(diag => {
    console.error(`${diag.severity}: ${diag.message}`);
  });
}
```

## What is HTMS?

HTMS is a declarative UI language that compiles to TypeScript with pure DOM API calls. It provides:

- **Compile-time Safety** - Component references validated at compile time
- **Type-Safe** - Full TypeScript output with type annotations
- **XSS-Safe** - Pure DOM API, no innerHTML
- **Zero Runtime** - Compiles to vanilla TypeScript/JavaScript

## Generated Files

The compiler generates three TypeScript files:

1. **templates.ts** - Component render functions using DOM API
2. **router.ts** - SPA routing with hash-based navigation
3. **events.ts** - Global event bus and state management

## Language Features

### Components

```htms
component Card(title: string, children: slot) {
  div [class: "card"] {
    h2 { {{ title }} }
    @slot
  }
}
```

### Conditional Rendering

```htms
@if ctx.isLoggedIn {
  p { {{ "Welcome back!" }} }
} @else {
  p { {{ "Please log in" }} }
}
```

### List Rendering

```htms
@each ctx.items as item, index {
  div { {{ item.name }} }
}
```

### Event Handling

```htms
button [onClick: handleClick] {
  {{ "Click Me" }}
}
```

### Routing

```htms
page home "/" {
  h1 { {{ "Home Page" }} }
}

page about "/about" {
  h1 { {{ "About Us" }} }
}
```

## CLI Tool

For command-line usage, install the CLI:

```bash
npm install -g @progalaxyelabs/htms-cli

htms compile app.htms -o dist/
htms compile app.htms --watch
htms check app.htms
```

## VSCode Extension

Get syntax highlighting and diagnostics:

- Search for "HTMS Language Support" in VSCode Extensions
- Or install from: [VSCode Marketplace](https://marketplace.visualstudio.com/items?itemName=progalaxy-labs.htms-vscode)

## API Reference

### `compile(source: string, options?: CompileOptions): CompileResult`

Compiles HTMS source code to TypeScript.

**Parameters:**
- `source` - HTMS source code string
- `options` - Optional compilation options
  - `generate_router` - Generate router.ts (default: true)
  - `generate_events` - Generate events.ts (default: true)

**Returns:**
- `success` - Boolean indicating compilation success
- `files` - Array of generated files with `path` and `content`
- `diagnostics` - Array of errors/warnings with `severity`, `message`, `location`

## Architecture

HTMS follows a traditional compiler pipeline:

1. **Lexer** - Tokenization using Logos
2. **Parser** - Recursive descent parser generating AST
3. **Analyzer** - Semantic analysis with symbol table
4. **Code Generator** - TypeScript code generation

## Rust/WASM

This package is written in Rust and compiled to WebAssembly for Node.js. This provides:

- **Fast compilation** - Native performance
- **Memory safety** - Rust's guarantees
- **Cross-platform** - Runs anywhere Node.js runs

## Links

- [GitHub Repository](https://github.com/progalaxyelabs/htms)
- [Documentation](https://github.com/progalaxyelabs/htms/tree/main/docs)
- [Issue Tracker](https://github.com/progalaxyelabs/htms/issues)
- [CLI Package](https://www.npmjs.com/package/@progalaxyelabs/htms-cli)

## License

MIT © ProGalaxy Labs
