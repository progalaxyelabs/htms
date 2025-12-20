# HTMS - HTM Script

A declarative language for building web UIs with compile-time safety.

[![npm version](https://img.shields.io/npm/v/@progalaxyelabs/htms-compiler.svg)](https://www.npmjs.com/package/@progalaxyelabs/htms-compiler)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## What is HTMS?

HTMS (HTM Script) is a modern, declarative language that compiles to TypeScript with pure DOM API calls. It combines the best of HTML's declarative syntax with compile-time validation and type safety.

```htms
component Button(text: string, onClick: function) {
  button [onClick: onClick, class: "btn-primary"] {
    {{ text }}
  }
}

page home "/" {
  div [class: "container"] {
    h1 { {{ "Welcome to HTMS!" }} }
    Button(text: "Click Me", onClick: handleClick)
  }
}
```

## Features

- 🔒 **Compile-time Safety** - Component references validated at compile time
- ⚡ **Zero Runtime** - Compiles to vanilla TypeScript/JavaScript
- 🎯 **DOM API First** - No innerHTML, no XSS vulnerabilities
- 🔄 **Two-way Binding** - Built-in support for form inputs
- 🛠️ **Great Tooling** - VSCode extension with syntax highlighting and diagnostics
- 📦 **Framework Agnostic** - Works with any backend or frontend setup

## Quick Start

### Installation

```bash
# Install the CLI
npm install -g @progalaxyelabs/htms-cli

# Or use with npx
npx @progalaxyelabs/htms-cli compile app.htms
```

### Your First HTMS File

Create `app.htms`:

```htms
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] { {{ "Home" }} }
    a [href: "#/about"] { {{ "About" }} }
  }
}

page home "/" {
  NavBar

  main [class: "container"] {
    h1 { {{ ctx.title }} }

    @if ctx.showGreeting {
      p { {{ "Hello, " + ctx.user.name }} }
    }

    @each ctx.items as item, index {
      div [class: "item"] {
        span { {{ item.name }} }
      }
    }
  }
}
```

### Compile

```bash
htms compile app.htms -o dist/
```

This generates:
- `dist/templates.ts` - Component render functions
- `dist/router.ts` - SPA routing
- `dist/events.ts` - Event system

## Language Features

### Components

Reusable UI components with parameters:

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
@each ctx.users as user, index {
  div [class: "user"] {
    span { {{ index + 1 }}. {{ user.name }} }
  }
}
```

### Event Handling

```htms
button [onClick: handleClick] {
  {{ "Click Me" }}
}

input [onInput: handleInput, bind: ctx.username]
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

## Packages

This monorepo contains:

| Package | Description | npm |
|---------|-------------|-----|
| [@progalaxyelabs/htms-compiler](htms-compiler/) | Rust/WASM compiler | [![npm](https://img.shields.io/npm/v/@progalaxyelabs/htms-compiler.svg)](https://www.npmjs.com/package/@progalaxyelabs/htms-compiler) |
| [@progalaxyelabs/htms-cli](htms-compiler/cli/) | CLI tools & Vite plugin | [![npm](https://img.shields.io/npm/v/@progalaxyelabs/htms-cli.svg)](https://www.npmjs.com/package/@progalaxyelabs/htms-cli) |
| [htms-vscode](htms-vscode/) | VSCode extension | [Marketplace](https://marketplace.visualstudio.com/items?itemName=progalaxy-labs.htms-vscode) |

## Development

### Building the Compiler

```bash
cd htms-compiler

# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

# Build WASM
wasm-pack build --target nodejs

# Run tests
cargo test
```

### Building the CLI

```bash
cd htms-compiler/cli

npm install
node bin/htms.js --help
```

### Building the VSCode Extension

```bash
cd htms-vscode

npm install
npm run compile
npm run package  # Creates .vsix file
```

## Usage with Vite

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import { htmsPlugin } from '@progalaxyelabs/htms-cli/vite';

export default defineConfig({
  plugins: [
    htmsPlugin({
      include: /\.htms$/,
      outputDir: 'src/generated'
    })
  ]
});
```

## CLI Commands

```bash
# Compile a file
htms compile app.htms -o dist/

# Watch mode
htms compile app.htms --watch

# Check for errors without output
htms check app.htms

# Get help
htms --help
```

## Architecture

HTMS follows a traditional compiler pipeline:

```
.htms file
    ↓
Lexer (tokenization)
    ↓
Parser (AST generation)
    ↓
Analyzer (validation)
    ↓
Code Generator
    ↓
TypeScript files (templates.ts, router.ts, events.ts)
```

### Why DOM API?

HTMS generates pure DOM API calls instead of innerHTML or template strings:

- ✅ **XSS-safe by default** - No string concatenation
- ✅ **Type-safe** - TypeScript compilation catches errors
- ✅ **Debuggable** - Step through generated code
- ✅ **Performant** - Direct DOM manipulation

## Documentation

- [Language Reference](docs/) - Full language syntax
- [CLI Guide](htms-compiler/cli/README.md) - Command-line usage
- [Publishing Guide](PUBLISHING.md) - How to publish packages

## Contributing

Contributions are welcome! Please read our contributing guidelines.

## License

MIT © ProGalaxy Labs

## Credits

Built with:
- [Rust](https://www.rust-lang.org/) - Compiler implementation
- [Logos](https://github.com/maciejhirsz/logos) - Fast lexer generator
- [wasm-pack](https://github.com/rustwasm/wasm-pack) - WebAssembly packaging
- [Commander.js](https://github.com/tj/commander.js) - CLI framework

---

**ProGalaxy Labs** | [GitHub](https://github.com/progalaxy-labs) | [Website](https://progalaxy.dev)
