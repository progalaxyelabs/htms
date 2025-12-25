---
layout: home

hero:
  name: HTMS
  text: HTM Script
  tagline: A declarative language for building web UIs with compile-time safety
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: AI Agent Guide
      link: /AI-AGENT-GUIDE
    - theme: alt
      text: View on GitHub
      link: https://github.com/progalaxyelabs/htms

features:
  - icon: 🛡️
    title: XSS-Safe by Default
    details: Pure DOM API - no innerHTML, no template strings. Zero injection vulnerabilities.
  - icon: ⚡
    title: Zero Runtime Overhead
    details: Compiles to vanilla JavaScript with pure document.createElement() calls
  - icon: 🔒
    title: Compile-time Safety
    details: Component references validated at compile time - catch errors before runtime
  - icon: 🎯
    title: Element Directives
    details: '@for and @if directives generate optimized, reusable functions'
  - icon: 🔄
    title: Two-way Binding
    details: Built-in form input binding with automatic synchronization
  - icon: 📦
    title: Dual Output Modes
    details: Generate TypeScript for SPAs or HTML for static sites
  - icon: 🛠️
    title: Great Tooling
    details: VSCode extension with syntax highlighting and diagnostics
  - icon: 🎨
    title: Clean Syntax
    details: Intuitive HTML-like syntax that's easy to learn and read
---

## Quick Example

```htms
component TodoItem(item: todo) {
  li [class: todo.done ? "completed" : ""] {
    input [
      type: "checkbox",
      checked: todo.done,
      onChange: toggleTodo(todo.id)
    ]
    span {{ ${todo.text} }}
    button [onClick: deleteTodo(todo.id)] {{ Delete }}
  }
}

page todos "/" {
  h1 {{ My Todos }}

  form [onSubmit.prevent: addTodo] {
    input [type: "text", bind: ctx.newTodo, placeholder: "What needs to be done?"]
    button {{ Add }}
  }

  ul @for(ctx.todos as todo) {
    TodoItem(item: todo)
  }
}
```

**Compiles to pure TypeScript with DOM API calls:**

```typescript
export function TodoItem(ctx: Context, todo: any): HTMLElement {
  const el0 = document.createElement('li');
  el0.className = todo.done ? 'completed' : '';

  const el1 = document.createElement('input');
  el1.type = 'checkbox';
  el1.checked = todo.done;
  // Event handling via data attributes
  el1.dataset.action = 'toggleTodo';
  el1.dataset.args = JSON.stringify([todo.id]);
  el0.appendChild(el1);

  // ... more pure DOM operations
  return el0;
}
```

## Why HTMS?

### Traditional Approach (Manual DOM)
```javascript
function createTodoItem(todo) {
  const li = document.createElement('li');
  li.className = todo.done ? 'completed' : '';

  const checkbox = document.createElement('input');
  checkbox.type = 'checkbox';
  checkbox.checked = todo.done;
  checkbox.onclick = () => toggleTodo(todo.id);
  li.appendChild(checkbox);

  const span = document.createElement('span');
  span.textContent = todo.text;
  li.appendChild(span);

  const button = document.createElement('button');
  button.textContent = 'Delete';
  button.onclick = () => deleteTodo(todo.id);
  li.appendChild(button);

  return li;
}
```

### With HTMS
```htms
component TodoItem(item: todo) {
  li [class: todo.done ? "completed" : ""] {
    input [type: "checkbox", checked: todo.done, onChange: toggleTodo(todo.id)]
    span {{ ${todo.text} }}
    button [onClick: deleteTodo(todo.id)] {{ Delete }}
  }
}
```

**70% less code. 100% safer. Compile-time validated.**

## Features at a Glance

- ✅ **Pure DOM API** - No innerHTML, no XSS vulnerabilities
- ✅ **Zero Runtime** - Compiles to vanilla JavaScript
- ✅ **TypeScript Output** - Full type safety
- ✅ **HTML Output** - For static sites
- ✅ **Element Directives** - `@for` and `@if` for optimized rendering
- ✅ **Two-way Binding** - `bind: ctx.value` just works
- ✅ **Hash Routing** - Built-in client-side navigation
- ✅ **Event Modifiers** - `.prevent`, `.stop`, `.once`
- ✅ **VSCode Extension** - Syntax highlighting and diagnostics
- ✅ **CLI Tools** - Compile, watch, check commands

## Get Started in 30 Seconds

```bash
npm install -g @progalaxyelabs/htms-cli
htms compile app.htms -o dist/
```

[Read the Getting Started Guide →](/guide/getting-started)

## Documentation

- **[AI Agent Quick Reference](/AI-AGENT-GUIDE)** - Complete guide for AI agents
- **[Language Reference](/guide/language-reference)** - Full syntax documentation
- **[Component Patterns](/guide/component-patterns)** - Best practices
- **[Examples](/examples/todo-app)** - Real-world applications
- **[API Reference](/api/generated-code)** - Understanding generated code
