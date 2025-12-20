# HTMS - HTML Script Language

## High-Level Design Document

**Version:** 0.1.0
**License:** MIT
**Status:** Design Complete, Implementation Pending

---

## Table of Contents

1. [Overview](#overview)
2. [Problem Statement](#problem-statement)
3. [Design Goals](#design-goals)
4. [Language Syntax](#language-syntax)
5. [Architecture](#architecture)
6. [Project Structure](#project-structure)
7. [Implementation Phases](#implementation-phases)
8. [Generated Output](#generated-output)
9. [Examples](#examples)
10. [Handoff Documents](#handoff-documents)

---

## Overview

HTMS (HTML Script) is a declarative domain-specific language for building web user interfaces. It compiles to **TypeScript template functions** (no Handlebars needed), providing a clean, intuitive syntax that combines the best aspects of HTML structure with modern programming language ergonomics.

**Implementation:** Rust + WebAssembly for cross-platform compatibility and performance.

### Key Features

- **Clean Syntax** - Brackets for attributes `[]`, braces for children `{}`
- **Compile-time Safety** - Component references are validated at compile time
- **Two-way Binding** - Built-in support for form input binding
- **Event Handling** - Declarative event handlers with action references
- **VSCode Support** - Syntax highlighting, error checking, autocomplete
- **Zero Dependencies** - Compiles to pure TypeScript, no template engine needed
- **Rust + WASM** - Fast, portable, works everywhere (Node, Browser, Deno)

---

## Problem Statement

Current web UI configuration approaches have limitations:

| Approach | Problem |
|----------|---------|
| Raw HTML | No reusable components, no data binding |
| JSON Config | Can't express ordered children + named attributes together |
| JS/TS Config | Looks like code, magic strings for references |
| JSX | Requires React runtime, not template-based |
| Vue SFC | Framework-specific, complex toolchain |

**HTMS solves this by providing:**
- Ordered children (like arrays) + named attributes (like objects) in one syntax
- Compile-time component reference validation (no magic strings)
- Compiles to pure TypeScript template functions (no runtime dependencies)
- Framework-agnostic, works with vanilla JS

---

## Design Goals

1. **Intuitive Syntax** - Developers should feel "yeah, that's what I would have done"
2. **Familiar Patterns** - Borrows from CSS, JS, and HTML conventions
3. **Compile-time Safety** - Catch errors before runtime
4. **Minimal Learning Curve** - If you know HTML and CSS, you know 80% of HTMS
5. **IDE First** - Great tooling from day one
6. **Extensible** - Plugin architecture for custom output targets

---

## Language Syntax

### Syntax Overview

```htms
// Component declaration
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] { {{ Home }} }
    a [href: "#/about"] { {{ About }} }
  }
}

// Page declaration with route
page home "/" {
  NavBar
  main {
    h1 { ctx.title }
  }
}
```

### Complete Syntax Reference

#### Elements

```htms
// Element with attributes
div [class: "container", id: "main"]

// Element with children
div {
  span { {{ Hello }} }
}

// Element with both
div [class: "card"] {
  h2 { {{ Title }} }
  p { {{ Description }} }
}

// Self-closing (no children block)
img [src: "logo.png", alt: "Logo"]
hr [class: "divider"]
input [type: "text", name: "email"]
```

#### Attributes

```htms
// Static attributes
div [class: "container", id: "main", data-testid: "wrapper"]

// Dynamic attributes (context binding)
div [class: ctx.isActive ? "active" : "inactive"]
span [title: ctx.user.name]

// Boolean attributes
input [type: "checkbox", checked: true, disabled: ctx.isLoading]

// Multi-line attributes (comma-separated)
input [
  type: "email",
  name: "user_email",
  placeholder: "you@example.com",
  required: true,
  disabled: ctx.form.isSubmitting
]
```

#### Text Content

```htms
// Static text (double braces, no quotes needed)
h1 { {{ Welcome to our site }} }
p { {{ He said "Hello" and she replied 'Hi!' }} }

// Dynamic text (context binding)
span { ctx.user.name }

// Mixed content
p {
  {{ Hello, }}
  strong { ctx.user.name }
  {{ ! Welcome back. }}
}

// Multi-line text
pre [class: "code-block"] {
  {{
    function greet(name) {
      return `Hello, ${name}!`;
    }
  }}
}
```

#### Components

```htms
// Component declaration
component Card [class: "card"] {
  div [class: "card-body"] {
    @slot  // Placeholder for children
  }
}

// Component with parameters
component UserCard (item: user) {
  div [class: "user-card"] {
    img [src: user.avatar]
    h3 { user.name }
    p { user.email }
  }
}

// Component usage (PascalCase, no quotes)
div {
  NavBar           // Simple reference
  Card {           // With children (fills @slot)
    p { {{ Content }} }
  }
  UserCard (item: ctx.currentUser)  // With parameter
}
```

#### Sections and Pages

```htms
// Section (reusable page fragment)
section HeroSection {
  div [class: "hero"] {
    h1 { ctx.hero.title }
    p { ctx.hero.subtitle }
    button [onClick: getStarted] { {{ Get Started }} }
  }
}

// Page with route
page home "/" {
  NavBar
  HeroSection
  Footer
}

page about "/about" {
  NavBar
  main { {{ About us content }} }
  Footer
}

page userProfile "/users/:id" {
  NavBar
  UserProfile
  Footer
}
```

#### Control Flow

```htms
// Conditional rendering
@if ctx.isLoggedIn {
  span { {{ Welcome back! }} }
} @else {
  a [href: "/login"] { {{ Sign In }} }
}

// Conditional with else-if
@if ctx.status == "loading" {
  Spinner
} @else if ctx.status == "error" {
  ErrorMessage
} @else {
  Content
}

// Loops
ul {
  @each ctx.items as item {
    li { item.name }
  }
}

// Loop with index
ul {
  @each ctx.items as item, index {
    li {
      span [class: "index"] { index }
      span { item.name }
    }
  }
}

// Nested loops
@each ctx.categories as category {
  h2 { category.name }
  ul {
    @each category.items as item {
      li { item.name }
    }
  }
}

// Empty state
@if ctx.items.length == 0 {
  p [class: "empty"] { {{ No items found }} }
}
```

#### Data Binding

```htms
// One-way binding (display only)
input [type: "text", value: ctx.form.name]

// Two-way binding (auto-updates context)
input [type: "text", bind: ctx.form.name]

// bind is equivalent to:
// value: ctx.form.name + onInput that updates ctx.form.name
```

#### Event Handling

```htms
// Simple event -> action
button [onClick: submitForm] { {{ Submit }} }

// Form events
form [onSubmit: handleSubmit] {
  input [bind: ctx.form.email]
  button { {{ Send }} }
}

// Multiple events
input [
  onFocus: trackFocus,
  onBlur: validateField,
  onInput: updateValue
]

// Event with parameters
button [onClick: selectTab("home")] { {{ Home }} }

@each ctx.items as item {
  button [onClick: selectItem(item.id)] { item.name }
}

// Event modifiers
form [onSubmit.prevent: handleSubmit]      // preventDefault
button [onClick.stop: doSomething]          // stopPropagation
a [onClick.prevent.stop: navigate]          // Both
button [onClick.once: showIntro]            // Remove after first trigger
```

#### Comments

```htms
// Single-line comment

/*
  Multi-line
  comment
*/

component NavBar {
  // This is the main navigation
  nav [class: "navbar"] {
    /* Logo section */
    div [class: "logo"] { {{ Brand }} }
  }
}
```

### Syntax Summary Table

| Construct | Syntax | Example |
|-----------|--------|---------|
| Element | `tag` | `div`, `span`, `input` |
| Attributes | `[key: value]` | `[class: "foo", id: "bar"]` |
| Children | `{ ... }` | `div { span { } }` |
| Text | `{{ text }}` | `{{ Hello "world" }}` |
| Dynamic | `ctx.path` | `ctx.user.name` |
| Component ref | `PascalCase` | `NavBar`, `UserCard` |
| Component param | `(item: value)` | `Card (item: ctx.user)` |
| Slot | `@slot` | Placeholder for children |
| Conditional | `@if ... @else` | `@if ctx.show { }` |
| Loop | `@each ... as` | `@each ctx.items as item` |
| Binding | `bind: ctx.path` | `bind: ctx.form.name` |
| Event | `onEvent: action` | `onClick: submit` |
| Event param | `onEvent: action(arg)` | `onClick: select(item.id)` |
| Modifier | `onEvent.mod: action` | `onClick.prevent: nav` |
| Comment | `//` or `/* */` | `// comment` |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         HTMS Ecosystem                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────┐     ┌──────────────────┐                  │
│  │  htms-vscode     │     │  htms-compiler   │                  │
│  │  (Extension)     │     │  (Core Package)  │                  │
│  │                  │     │                  │                  │
│  │  - Syntax HL     │     │  - Lexer         │                  │
│  │  - Diagnostics   │     │  - Parser        │                  │
│  │  - Autocomplete  │     │  - AST           │                  │
│  │  - Go-to-def     │     │  - Analyzer      │                  │
│  │                  │     │  - Code Gen      │                  │
│  └────────┬─────────┘     └────────┬─────────┘                  │
│           │                        │                             │
│           └────────┬───────────────┘                             │
│                    │                                             │
│                    ▼                                             │
│  ┌──────────────────────────────────────────┐                   │
│  │         Language Server (LSP)             │                   │
│  │         (Shared Protocol)                 │                   │
│  └──────────────────────────────────────────┘                   │
│                    │                                             │
│                    ▼                                             │
│  ┌──────────────────────────────────────────┐                   │
│  │         htms-runtime                      │                   │
│  │         (Browser Runtime)                 │                   │
│  │                                           │                   │
│  │  - Event delegation                       │                   │
│  │  - Two-way binding                        │                   │
│  │  - Context management                     │                   │
│  └──────────────────────────────────────────┘                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Compiler Pipeline

```
┌─────────┐    ┌────────┐    ┌─────┐    ┌──────────┐    ┌─────────┐
│  .htms  │───▶│ Lexer  │───▶│ AST │───▶│ Analyzer │───▶│ CodeGen │
│  Source │    │        │    │     │    │          │    │         │
└─────────┘    └────────┘    └─────┘    └──────────┘    └─────────┘
                   │                         │               │
                   ▼                         ▼               ▼
              Token Stream            Symbol Table      Output Files
                                      Error List        .hbs + .ts
```

---

## Project Structure

```
/ssd2/projects/progalaxy-elabs/opensource/htms/
│
├── htms-compiler/              # Core compiler (npm package)
│   ├── package.json
│   ├── tsconfig.json
│   ├── LICENSE                 # MIT
│   ├── HLD.md                  # This document
│   ├── src/
│   │   ├── index.ts            # Main exports
│   │   ├── cli.ts              # CLI entry point
│   │   ├── lexer/
│   │   │   ├── index.ts
│   │   │   ├── tokens.ts       # Token type definitions
│   │   │   └── scanner.ts      # Character scanner
│   │   ├── parser/
│   │   │   ├── index.ts
│   │   │   ├── ast.ts          # AST node types
│   │   │   └── grammar.ts      # Recursive descent parser
│   │   ├── analyzer/
│   │   │   ├── index.ts
│   │   │   ├── symbols.ts      # Symbol table
│   │   │   ├── resolver.ts     # Reference resolution
│   │   │   └── validator.ts    # Semantic validation
│   │   ├── codegen/
│   │   │   ├── index.ts
│   │   │   ├── hbs.ts          # Handlebars generator
│   │   │   ├── router.ts       # Router.ts generator
│   │   │   ├── events.ts       # Events.ts generator
│   │   │   └── types.ts        # TypeScript types generator
│   │   └── errors/
│   │       ├── index.ts
│   │       └── diagnostic.ts   # Error formatting
│   └── tests/
│       ├── lexer.test.ts
│       ├── parser.test.ts
│       ├── analyzer.test.ts
│       └── codegen.test.ts
│
├── htms-vscode/                # VSCode extension
│   ├── package.json
│   ├── LICENSE
│   ├── syntaxes/
│   │   └── htms.tmLanguage.json
│   ├── language-configuration.json
│   └── src/
│       ├── extension.ts
│       └── server.ts           # Language server
│
├── htms-runtime/               # Browser runtime (npm package)
│   ├── package.json
│   ├── LICENSE
│   └── src/
│       ├── index.ts
│       ├── events.ts           # Event delegation
│       ├── binding.ts          # Two-way binding
│       └── context.ts          # Context management
│
├── htms-docs/                  # Documentation website
│   ├── package.json
│   └── src/
│       ├── pages/
│       ├── components/
│       └── examples/
│
└── htms-examples/              # Example projects
    ├── marketing-site/
    ├── todo-app/
    └── dashboard/
```

---

## Implementation Phases

### Phase 1: Core Lexer & Parser

**Goal:** Parse HTMS source into AST

**Deliverables:**
- Token definitions (keywords, identifiers, operators, literals)
- Lexer/Scanner with line/column tracking
- Recursive descent parser
- AST node types
- Basic error messages with location

**Files to create:**
- `src/lexer/tokens.ts`
- `src/lexer/scanner.ts`
- `src/lexer/index.ts`
- `src/parser/ast.ts`
- `src/parser/grammar.ts`
- `src/parser/index.ts`
- `tests/lexer.test.ts`
- `tests/parser.test.ts`

### Phase 2: Semantic Analyzer

**Goal:** Validate AST and resolve references

**Deliverables:**
- Symbol table (components, sections, pages)
- Reference resolver (NavBar → component definition)
- Error detection (undefined component, duplicate names)
- Context path validation

**Files to create:**
- `src/analyzer/symbols.ts`
- `src/analyzer/resolver.ts`
- `src/analyzer/validator.ts`
- `src/analyzer/index.ts`
- `tests/analyzer.test.ts`

### Phase 3: Code Generation

**Goal:** Generate HBS + TS from validated AST

**Deliverables:**
- Handlebars template generator
- Router.ts generator
- Events.ts generator
- TypeScript interface generator

**Files to create:**
- `src/codegen/hbs.ts`
- `src/codegen/router.ts`
- `src/codegen/events.ts`
- `src/codegen/types.ts`
- `src/codegen/index.ts`
- `tests/codegen.test.ts`

### Phase 4: CLI & Integration

**Goal:** Command-line tool and build integration

**Deliverables:**
- CLI with `htms compile`, `htms watch` commands
- Vite plugin
- Error formatting for terminal

**Files to create:**
- `src/cli.ts`
- `src/vite-plugin.ts`
- `src/errors/diagnostic.ts`

### Phase 5: VSCode Extension

**Goal:** IDE support with syntax highlighting and diagnostics

**Deliverables:**
- TextMate grammar for syntax highlighting
- Language configuration (brackets, comments)
- Language server for diagnostics
- Autocomplete for tags and components
- Go-to-definition for component references

**Files to create:**
- `htms-vscode/syntaxes/htms.tmLanguage.json`
- `htms-vscode/language-configuration.json`
- `htms-vscode/src/extension.ts`
- `htms-vscode/src/server.ts`

### Phase 6: Runtime & Documentation

**Goal:** Browser runtime and marketing site

**Deliverables:**
- htms-runtime package
- Documentation website
- Example projects

---

## Generated Output

### Input (app.htms)

```htms
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] { {{ Home }} }
    a [href: "#/about"] { {{ About }} }
  }
}

component ContactForm {
  form [onSubmit.prevent: submitContact] {
    label { {{ Name }} }
    input [type: "text", bind: ctx.form.name, required: true]

    label { {{ Email }} }
    input [type: "email", bind: ctx.form.email, required: true]

    button { {{ Send }} }

    @if ctx.form.success {
      div [class: "success"] { {{ Thanks! We'll be in touch. }} }
    }
  }
}

page home "/" {
  NavBar
  main {
    h1 { ctx.home.title }
    p { ctx.home.subtitle }
  }
}

page contact "/contact" {
  NavBar
  ContactForm
}
```

### Output Files

**components/navBar.hbs:**
```handlebars
<nav class="navbar">
  <a href="#/">Home</a>
  <a href="#/about">About</a>
</nav>
```

**components/contactForm.hbs:**
```handlebars
<form data-action="submitContact" data-event="submit" data-prevent="true">
  <label>Name</label>
  <input type="text" value="{{form.name}}" data-bind="form.name" required>

  <label>Email</label>
  <input type="email" value="{{form.email}}" data-bind="form.email" required>

  <button>Send</button>

  {{#if form.success}}
    <div class="success">Thanks! We'll be in touch.</div>
  {{/if}}
</form>
```

**pages/home.hbs:**
```handlebars
{{> navBar}}
<main>
  <h1>{{home.title}}</h1>
  <p>{{home.subtitle}}</p>
</main>
```

**pages/contact.hbs:**
```handlebars
{{> navBar}}
{{> contactForm}}
```

**router.ts:**
```typescript
import { Router } from '@aspect/htms-runtime';
import Handlebars from 'handlebars';

// Import templates
import navBarTemplate from './components/navBar.hbs?raw';
import contactFormTemplate from './components/contactForm.hbs?raw';
import homeTemplate from './pages/home.hbs?raw';
import contactTemplate from './pages/contact.hbs?raw';

// Register partials
Handlebars.registerPartial('navBar', navBarTemplate);
Handlebars.registerPartial('contactForm', contactFormTemplate);

// Create router
export const router = new Router({
  mode: 'hash',
  routes: {
    '/': () => renderPage('home', homeTemplate),
    '/contact': () => renderPage('contact', contactTemplate)
  }
});
```

**events.ts:**
```typescript
import { initEventDelegation, initBinding } from '@aspect/htms-runtime';
import { actions } from './actions';

export function initEvents() {
  initEventDelegation(actions);
  initBinding();
}
```

---

## Examples

### Marketing Website

```htms
// components
component NavBar {
  nav [class: "navbar navbar-expand-lg"] {
    div [class: "container"] {
      a [class: "navbar-brand", href: "#/"] {
        img [src: "/logo.svg", alt: ctx.app.name]
      }
      ul [class: "navbar-nav ms-auto"] {
        li { a [class: "nav-link", href: "#/features"] { {{ Features }} } }
        li { a [class: "nav-link", href: "#/pricing"] { {{ Pricing }} } }
        li { a [class: "nav-link", href: "#/docs"] { {{ Docs }} } }
        li { a [class: "btn btn-primary", href: "#/signup"] { {{ Get Started }} } }
      }
    }
  }
}

component HeroSection {
  section [class: "hero"] {
    div [class: "container"] {
      h1 [class: "display-4"] { ctx.hero.title }
      p [class: "lead"] { ctx.hero.subtitle }
      div [class: "cta-buttons"] {
        a [class: "btn btn-primary btn-lg", href: "#/signup"] { {{ Start Free }} }
        a [class: "btn btn-outline-primary btn-lg", href: "#/demo"] { {{ Watch Demo }} }
      }
    }
  }
}

component FeatureCard (item: feature) {
  div [class: "col-md-4"] {
    div [class: "card h-100"] {
      div [class: "card-body text-center"] {
        i [class: feature.icon]
        h3 { feature.title }
        p { feature.description }
      }
    }
  }
}

component FeaturesSection {
  section [class: "features py-5"] {
    div [class: "container"] {
      h2 [class: "text-center mb-5"] { {{ Why Choose HTMS? }} }
      div [class: "row g-4"] {
        @each ctx.features as feature {
          FeatureCard (item: feature)
        }
      }
    }
  }
}

component Footer {
  footer [class: "footer py-4"] {
    div [class: "container text-center"] {
      p { {{ © 2025 }} ctx.app.name {{ . All rights reserved. }} }
    }
  }
}

// pages
page home "/" {
  NavBar
  HeroSection
  FeaturesSection
  Footer
}
```

### Todo Application

```htms
component TodoItem (item: todo) {
  li [class: todo.done ? "todo-item done" : "todo-item"] {
    input [
      type: "checkbox",
      checked: todo.done,
      onChange: toggleTodo(todo.id)
    ]

    @if ctx.editing == todo.id {
      input [
        type: "text",
        bind: todo.text,
        onBlur: saveEdit(todo.id),
        onKeydown.enter: saveEdit(todo.id)
      ]
    } @else {
      span [onDblClick: startEdit(todo.id)] { todo.text }
    }

    button [class: "delete", onClick: deleteTodo(todo.id)] {
      i [class: "bi bi-trash"]
    }
  }
}

component TodoApp {
  div [class: "todo-app"] {
    h1 { {{ My Todos }} }

    form [onSubmit.prevent: addTodo] {
      input [
        type: "text",
        bind: ctx.newTodo,
        placeholder: "What needs to be done?"
      ]
      button { {{ Add }} }
    }

    nav [class: "filters"] {
      @each ctx.filterOptions as filter {
        button [
          class: ctx.filter == filter.value ? "active" : "",
          onClick: setFilter(filter.value)
        ] { filter.label }
      }
    }

    ul [class: "todo-list"] {
      @each ctx.filteredTodos as todo {
        TodoItem (item: todo)
      }
    }

    @if ctx.todos.length == 0 {
      p [class: "empty"] { {{ No todos yet! }} }
    }

    footer {
      span { ctx.activeCount {{ items left }} }
      @if ctx.doneCount > 0 {
        button [onClick: clearCompleted] { {{ Clear completed }} }
      }
    }
  }
}

page home "/" {
  TodoApp
}
```

### Data Table

```htms
component UserRow (item: user) {
  tr [class: user.active ? "active" : ""] {
    td { user.id }
    td {
      img [src: user.avatar, class: "avatar-sm"]
      user.name
    }
    td { user.email }
    td {
      span [class: "badge " + user.roleClass] { user.role }
    }
    td [class: "actions"] {
      button [onClick: editUser(user.id), title: "Edit"] {
        i [class: "bi bi-pencil"]
      }
      button [onClick: deleteUser(user.id), title: "Delete"] {
        i [class: "bi bi-trash"]
      }
    }
  }
}

component UserTable {
  div [class: "table-container"] {
    table [class: "table table-striped"] {
      thead {
        tr {
          th { {{ ID }} }
          th { {{ Name }} }
          th { {{ Email }} }
          th { {{ Role }} }
          th { {{ Actions }} }
        }
      }
      tbody {
        @each ctx.users as user {
          UserRow (item: user)
        }
      }
    }

    @if ctx.users.length == 0 {
      div [class: "empty-state"] {
        i [class: "bi bi-people"]
        p { {{ No users found }} }
      }
    }

    nav [class: "pagination"] {
      button [disabled: ctx.page == 1, onClick: prevPage] {
        {{ ← Previous }}
      }
      span { {{ Page }} ctx.page {{ of }} ctx.totalPages }
      button [disabled: ctx.page == ctx.totalPages, onClick: nextPage] {
        {{ Next → }}
      }
    }
  }
}
```

---

## Handoff Documents

See the `HANDOFF-*.md` files in this directory for implementation details for each phase.

---

## API Reference

### Compiler API

```typescript
import {
  parse,
  analyze,
  generate,
  compile
} from '@aspect/htms-compiler';

// Parse source to AST
const ast = parse(source);

// Analyze AST (resolve references, validate)
const { symbols, errors } = analyze(ast);

// Generate output files
const files = generate(ast, symbols);

// All-in-one compile
const result = compile(source);
// result.files: Array<{ path: string, content: string }>
// result.errors: Array<Diagnostic>
```

### CLI Usage

```bash
# Compile single file
htms compile src/app.htms -o dist/

# Compile directory
htms compile src/ -o dist/

# Watch mode
htms watch src/ -o dist/

# Check syntax only
htms check src/app.htms
```

### Vite Plugin

```typescript
// vite.config.ts
import { htmsPlugin } from '@aspect/htms-compiler/vite';

export default {
  plugins: [htmsPlugin()]
};
```

---

## License

MIT License - See LICENSE file for details.

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `npm test`
5. Submit a pull request

---

*This document is the source of truth for HTMS language design and implementation.*
