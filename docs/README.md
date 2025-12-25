# HTMS Documentation

Complete documentation for HTMS (HTM Script) - A declarative language for building web UIs.

## Quick Links

### For AI Agents
- **[AI Agent Quick Reference](./AI-AGENT-GUIDE.md)** - ⭐ START HERE for AI agents - Complete quick reference optimized for AI

### Getting Started
- [Getting Started](./guide/getting-started.md) - Installation, first app, CLI commands
- [Language Reference](./guide/language-reference.md) - Complete syntax guide
- [Component Patterns](./guide/component-patterns.md) - Best practices and common patterns
- [Build & Deploy](./guide/build-deploy.md) - Production deployment guide

### Examples
- [Todo App](./examples/todo-app.md) - Full-featured todo application
- [Instagram Clone](./examples/instagram-clone.md) - Social media app example

### API Reference
- [Generated Code](./api/generated-code.md) - Understanding compiler output

---

## What is HTMS?

HTMS (HTM Script) is a declarative language that compiles to:
- **TypeScript** with pure DOM API calls (XSS-safe, no innerHTML)
- **HTML** with client-side routing (for static sites)

### Key Features

✅ **Pure DOM API** - No innerHTML, no template strings
✅ **XSS-Safe by Default** - Zero injection vulnerabilities
✅ **Compile-time Safety** - Component references validated at build
✅ **Dual Output Modes** - TypeScript (dynamic) or HTML (static)
✅ **Zero Runtime** - Compiles to vanilla JavaScript
✅ **Element Directives** - `@for` and `@if` for optimized rendering
✅ **Two-way Binding** - Built-in form binding
✅ **Hash-based Routing** - Client-side navigation included

---

## Quick Example

```htms
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] {{ Home }}
    a [href: "#/about"] {{ About }}
  }
}

component TodoItem(item: todo) {
  li [class: todo.done ? "done" : ""] {
    input [
      type: "checkbox",
      checked: todo.done,
      onChange: toggleTodo(todo.id)
    ]
    span {{ ${todo.text} }}
    button [onClick: deleteTodo(todo.id)] {{ Delete }}
  }
}

page home "/" {
  NavBar
  main {
    h1 {{ My Todos }}

    form [onSubmit.prevent: addTodo] {
      input [type: "text", bind: ctx.newTodo]
      button {{ Add }}
    }

    ul @for(ctx.todos as todo) {
      TodoItem(item: todo)
    }
  }
}
```

**Compiles to pure TypeScript:**
```typescript
export function TodoItem(ctx: Context, todo: any): HTMLElement {
  const el0 = document.createElement('li');
  el0.className = todo.done ? 'done' : '';

  const el1 = document.createElement('input');
  el1.type = 'checkbox';
  el1.checked = todo.done;
  el1.dataset.action = 'toggleTodo';
  el1.dataset.args = JSON.stringify([todo.id]);
  el0.appendChild(el1);

  const el2 = document.createElement('span');
  const el3 = document.createTextNode(todo.text);
  el2.appendChild(el3);
  el0.appendChild(el2);

  // ... more DOM creation
  return el0;
}
```

---

## Documentation Structure

### 📖 Guides

#### [Getting Started](./guide/getting-started.md)
- Installation and setup
- Your first HTMS file
- CLI commands reference
- Using with build tools (Vite, Webpack)
- Common gotchas and troubleshooting

#### [Language Reference](./guide/language-reference.md)
Complete syntax documentation:
- Components, sections, pages
- Elements and attributes
- Text content and interpolation
- Control flow (`@if`, `@each`)
- Element directives (`@for`, `@if`)
- Data binding (one-way and two-way)
- Event handling with modifiers
- Context (ctx) usage

#### [Component Patterns](./guide/component-patterns.md)
Best practices and patterns:
- Component design principles
- Common patterns (cards, modals, navigation)
- State management
- Form handling
- List rendering strategies
- Code organization
- Performance tips
- Common mistakes

#### [Build & Deploy](./guide/build-deploy.md)
Production deployment:
- Build process for TypeScript and HTML outputs
- Deployment platforms (Netlify, Vercel, GitHub Pages, AWS)
- Build optimization (minification, code splitting)
- CI/CD pipelines
- Performance optimization
- Monitoring and analytics
- Security best practices

### 💡 Examples

#### [Todo Application](./examples/todo-app.md)
Full-featured todo app with:
- Add, edit, delete todos
- Mark as complete
- Filter (all, active, completed)
- LocalStorage persistence
- Complete source code with explanations

#### [Instagram Clone](./examples/instagram-clone.md)
Social media app with:
- Feed with posts and stories
- Like and bookmark functionality
- Post detail with comments
- User profile
- Bottom navigation
- Search functionality

### 🔧 API Reference

#### [Generated Code](./api/generated-code.md)
Understanding compiler output:
- TypeScript output format (templates.ts, router.ts, events.ts)
- HTML output format
- Component function signatures
- Element directive code generation
- Context management API
- Router API
- Event system
- Data attributes for functionality
- TypeScript type safety

---

## Quick Start (30 seconds)

```bash
# Install CLI
npm install -g @progalaxyelabs/htms-cli

# Create app.htms
cat > app.htms << 'EOF'
component Hello {
  div [class: "greeting"] {
    h1 {{ Hello, ${ctx.name}! }}
    button [onClick: changeName] {{ Change Name }}
  }
}

page home "/" {
  Hello
}
EOF

# Create actions.ts
cat > actions.ts << 'EOF'
export const actions = {
  changeName: (ctx, event) => {
    ctx.data.name = prompt('Enter name:') || 'World';
    ctx.rerender();
  }
};
EOF

# Compile
htms compile app.htms -o dist/

# Done! Import in your main.ts and run
```

---

## Installation

### NPM Packages

```bash
# CLI (recommended)
npm install -g @progalaxyelabs/htms-cli

# Or as dev dependency
npm install --save-dev @progalaxyelabs/htms-cli

# Compiler only (for programmatic use)
npm install --save-dev @progalaxyelabs/htms-compiler
```

### VSCode Extension

Search for "HTMS" in VSCode extensions marketplace, or:

```bash
code --install-extension progalaxyelabs.htms-vscode
```

Features:
- Syntax highlighting
- Error diagnostics
- Code snippets
- Language configuration

---

## Use Cases

### Perfect For:

✅ **Single Page Applications (SPAs)**
- Dashboard applications
- Admin panels
- Social media apps
- E-commerce frontends
- Productivity tools

✅ **Static Websites**
- Marketing sites
- Landing pages
- Documentation sites
- Portfolios
- Blogs

✅ **Mobile-First Web Apps**
- Progressive Web Apps (PWAs)
- Mobile-optimized UIs
- Touch-friendly interfaces

### When to Use HTMS vs Alternatives:

| Need | Use HTMS | Consider Alternative |
|------|----------|---------------------|
| XSS-safe by default | ✅ | React (JSX requires sanitization) |
| Zero runtime overhead | ✅ | Svelte ✓, React ✗ |
| Compile-time validation | ✅ | TypeScript + React ✓ |
| No build tool required | ✅ (HTML mode) | Plain HTML ✓ |
| Server-side rendering | ✗ | Next.js, SvelteKit |
| Complex state management | ✗ (use external library) | Redux, Zustand |
| Large existing ecosystem | ✗ | React, Vue |

---

## How It Works

```
┌─────────────┐
│  app.htms   │  Your declarative UI definition
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Lexer     │  Tokenization
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Parser    │  AST generation
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Analyzer   │  Validation & symbol resolution
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  CodeGen    │  Generate TypeScript or HTML
└──────┬──────┘
       │
       ├─────────────────────┬─────────────────────┐
       ▼                     ▼                     ▼
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│templates.ts │      │ router.ts   │      │ events.ts   │
└─────────────┘      └─────────────┘      └─────────────┘
       │                     │                     │
       └─────────────────────┴─────────────────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │  Your App       │
                    │  (Vanilla JS)   │
                    └─────────────────┘
```

---

## Philosophy

HTMS is built on these principles:

1. **Safety First** - XSS vulnerabilities eliminated by using pure DOM API
2. **Developer Experience** - Clean, intuitive syntax that's easy to learn
3. **Compile-time Validation** - Catch errors before runtime
4. **Performance** - Zero runtime overhead, compiles to vanilla JavaScript
5. **Simplicity** - Minimal concepts, maximum productivity
6. **Framework Agnostic** - Works with any backend or build tool

---

## Comparison with Other Technologies

### vs React

| Feature | HTMS | React |
|---------|------|-------|
| Runtime | Zero | ~40KB (React + ReactDOM) |
| XSS Safety | Built-in (DOM API) | Manual (requires sanitization) |
| Learning Curve | Low (HTML-like) | Medium (JSX, hooks, etc.) |
| Compile-time Validation | Yes | TypeScript (optional) |
| Ecosystem | New | Massive |

### vs Vue

| Feature | HTMS | Vue |
|---------|------|-----|
| Runtime | Zero | ~33KB |
| Template Syntax | `.htms` files | SFC `.vue` files |
| Reactivity | Manual (ctx.rerender) | Automatic (Proxy) |
| Tooling Requirement | CLI | Vite/Webpack |

### vs Svelte

| Feature | HTMS | Svelte |
|---------|------|--------|
| Runtime | Zero | ~2KB |
| Compile Target | Pure DOM API | Optimized DOM operations |
| Syntax | HTML-like with `{{ }}` | HTML-like with `{}` |
| Ecosystem | New | Growing |

### vs Plain HTML/JS

| Feature | HTMS | Plain HTML/JS |
|---------|------|---------------|
| Component Reusability | Built-in | Manual |
| Type Safety | TypeScript output | Manual |
| Routing | Included | DIY or library |
| Event Delegation | Auto-generated | Manual setup |
| Data Binding | Built-in | Manual |

---

## Community & Support

- **GitHub**: [github.com/progalaxyelabs/htms](https://github.com/progalaxyelabs/htms)
- **Issues**: [Report bugs or request features](https://github.com/progalaxyelabs/htms/issues)
- **NPM**: [@progalaxyelabs/htms-cli](https://www.npmjs.com/package/@progalaxyelabs/htms-cli)
- **License**: MIT

---

## Contributing

See the main repository's CONTRIBUTING.md for guidelines.

---

## Roadmap

### Completed ✅
- Lexer, parser, analyzer, code generator
- TypeScript output format
- HTML output format
- Element directives (`@for`, `@if`)
- Two-way data binding
- CLI with watch mode
- VSCode extension (basic)
- Event modifiers

### In Progress 🚧
- `htms init` command
- Enhanced VSCode features (go-to-definition, autocomplete)
- Documentation website

### Planned 📋
- TypeScript type generation for context
- Source maps
- CSS-in-JS support
- SSG (Static Site Generation)
- Plugin system

---

## FAQ

**Q: Is HTMS production-ready?**
A: Yes! The core compiler is feature-complete and stable.

**Q: Can I use HTMS with React/Vue?**
A: Not recommended. HTMS is a complete alternative, not a complement.

**Q: How do I handle API calls?**
A: In your `actions.ts` file using fetch/axios/etc.

**Q: Does HTMS support SSR?**
A: Not yet. HTML output mode provides static HTML, but no server-side rendering of dynamic data.

**Q: How do I add CSS?**
A: Write regular CSS files and link them in your HTML. HTMS handles UI structure, not styling.

**Q: Can I use TypeScript for actions?**
A: Yes! Just name your file `actions.ts` and use TypeScript.

**Q: How big is the generated code?**
A: Very small. Simple apps are ~5-10KB (minified). No framework runtime.

**Q: Is HTMS faster than React?**
A: Runtime performance is similar. HTMS has zero framework overhead, but no virtual DOM optimization.

---

## License

MIT License - See LICENSE file for details.

---

## Acknowledgments

Built by ProGalaxy eLabs with ❤️ for the web development community.

---

**Ready to start? Head to [Getting Started](./guide/getting-started.md) →**
