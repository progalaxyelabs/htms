# HTMS Quick Reference for AI Agents

This document is optimized for AI agents to quickly understand and build applications with HTMS.

## What is HTMS?

HTMS (HTM Script) is a declarative language that compiles to:
- **TypeScript** with pure DOM API calls (no innerHTML, XSS-safe)
- **HTML** with client-side routing (for static sites)

**Key Points:**
- Compiles to vanilla JavaScript - zero runtime overhead
- Component references validated at compile time
- Two-way data binding built-in
- Element directives for optimized rendering

---

## File Structure for a Project

```
project/
├── src/
│   ├── app.htms          # Your HTMS source
│   ├── main.ts           # App initialization
│   ├── actions.ts        # Event handlers (YOU write this)
│   ├── styles.css        # CSS styles
│   └── dist/             # Generated files
│       ├── templates.ts  # Generated component functions
│       ├── router.ts     # Generated router
│       └── events.ts     # Generated event system
├── index.html
└── package.json
```

---

## Complete Syntax Cheat Sheet

### Components

```htms
// Basic component
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] {{ Home }}
  }
}

// Component with parameter
component UserCard(item: user) {
  div [class: "card"] {
    h3 {{ ${user.name} }}
    p {{ ${user.email} }}
  }
}

// Component with slot
component Card {
  div [class: "card-body"] {
    @slot
  }
}
```

### Pages (Routes)

```htms
page home "/" {
  NavBar
  main {{ Welcome }}
}

page userProfile "/users/:id" {
  NavBar
  UserProfile
}
```

### Elements & Attributes

```htms
// Element with attributes
div [class: "container", id: "main"]

// Multi-line attributes
input [
  type: "email",
  placeholder: "you@example.com",
  required: true
]

// Dynamic attributes
div [class: ctx.isActive ? "active" : "inactive"]
img [src: ctx.user.avatar]

// Boolean attributes
input [type: "checkbox", checked: ctx.accepted]
button [disabled: ctx.isLoading]
```

### Text Content

```htms
// Static text (no quotes!)
h1 {{ Welcome to HTMS }}

// Dynamic text (use ${})
p {{ Hello, ${ctx.user.name}! }}

// Shorthand (no braces for text-only)
button {{ Click Me }}

// Verbose form
button {
  {{ Click Me }}
}
```

### Control Flow - Block Form

```htms
// If/else
@if ctx.isLoggedIn {
  div {{ Welcome back! }}
} @else {
  a [href: "#/login"] {{ Sign In }}
}

// If/else if/else
@if ctx.status == "loading" {
  div {{ Loading... }}
} @else if ctx.status == "error" {
  div {{ Error occurred }}
} @else {
  div {{ Success }}
}

// Each (loops)
ul {
  @each ctx.items as item {
    li {{ ${item.name} }}
  }
}

// Each with index
ul {
  @each ctx.items as item, index {
    li {{ ${index + 1}. ${item.name} }}
  }
}

// Nested loops
@each ctx.categories as category {
  h2 {{ ${category.name} }}
  @each category.items as item {
    p {{ ${item.name} }}
  }
}
```

### Element Directives (Optimized)

```htms
// @for directive (generates reusable function)
div @for(ctx.users as user) {
  div [class: "user-card"] {
    h3 {{ ${user.name} }}
  }
}

// @for with index
ul @for(ctx.items as item, index) {
  li {{ ${index + 1}. ${item.name} }}
}

// @if directive (generates reusable function)
div @if(ctx.showMessage) {
  p {{ ${ctx.message} }}
}

// Combining directives
div @if(ctx.items.length > 0) {
  ul @for(ctx.items as item) {
    li {{ ${item.name} }}
  }
}
```

**When to use:**
- Use `@for` / `@if` (element directives) when you have a single root element
- Use `@each` / `@if/@else` (block directives) when you need multiple siblings or else clauses

### Data Binding

```htms
// One-way binding (display only)
input [type: "text", value: ctx.form.name]

// Two-way binding (auto-updates ctx)
input [type: "text", bind: ctx.form.name]
textarea [bind: ctx.form.message]

// Two-way with checkbox
input [type: "checkbox", bind: ctx.form.accepted]
```

### Event Handling

```htms
// Basic event
button [onClick: handleClick] {{ Click }}

// Event with parameter
button [onClick: deleteItem(item.id)] {{ Delete }}

// Multiple events
input [
  onFocus: trackFocus,
  onBlur: validateField,
  onInput: updateValue
]

// Event modifiers
form [onSubmit.prevent: handleSubmit] {
  button {{ Submit }}
}

button [onClick.stop: doSomething] {{ Click }}
button [onClick.once: showWelcome] {{ One-time }}

// Multiple modifiers
a [onClick.prevent.stop: navigate] {{ Link }}
```

**Available modifiers:**
- `.prevent` - preventDefault()
- `.stop` - stopPropagation()
- `.once` - Remove after first trigger

### Comments

```htms
// Single-line comment

/*
  Multi-line
  comment
*/
```

---

## Required Files You Must Create

### 1. actions.ts (REQUIRED)

```typescript
export const actions = {
  // Simple action
  handleClick: (ctx, event) => {
    console.log('Clicked');
    ctx.data.count++;
    ctx.rerender();
  },

  // Action with parameter
  deleteItem: (id: number) => (ctx, event) => {
    ctx.data.items = ctx.data.items.filter(item => item.id !== id);
    ctx.rerender();
  },

  // Async action
  loadData: async (ctx, event) => {
    ctx.data.isLoading = true;
    ctx.rerender();

    const response = await fetch('/api/data');
    ctx.data.items = await response.json();
    ctx.data.isLoading = false;
    ctx.rerender();
  },

  // Form submission
  handleSubmit: (ctx, event) => {
    event.preventDefault();
    // Process ctx.data.form
    ctx.data.form.submitted = true;
    ctx.rerender();
  }
};
```

**Key points:**
- Export object named `actions`
- Each action receives `(ctx, event)` where:
  - `ctx.data` = your app state
  - `ctx.rerender()` = re-render function
  - `event` = DOM event
- For parameters, use curried functions: `(param) => (ctx, event) => { }`
- Always call `ctx.rerender()` after state changes

### 2. main.ts (REQUIRED)

```typescript
import { setContext, router } from './dist/router';
import { initEvents } from './dist/events';

// Initialize app state
setContext({
  user: { name: 'John', email: 'john@example.com' },
  items: [],
  form: { email: '', message: '' },
  isLoading: false
});

// Start event system and router
initEvents();
router.init();
```

### 3. runtime.ts (REQUIRED)

Simple router implementation:

```typescript
export class Router {
  private routes: Record<string, () => void>;
  private notFound: () => void;

  constructor(config: {
    mode: 'hash';
    routes: Record<string, () => void>;
    notFound: () => void;
  }) {
    this.routes = config.routes;
    this.notFound = config.notFound;
  }

  init() {
    window.addEventListener('hashchange', () => this.handleRoute());
    this.handleRoute();
  }

  private handleRoute() {
    const hash = window.location.hash.slice(1) || '/';
    const handler = this.routes[hash] || this.notFound;
    handler();
  }

  navigate(path: string) {
    window.location.hash = `#${path}`;
  }
}
```

### 4. index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>HTMS App</title>
  <link rel="stylesheet" href="/src/styles.css">
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

---

## CLI Commands

```bash
# Compile to TypeScript (default)
htms compile app.htms -o dist/

# Compile to HTML
htms compile app.htms -o dist/ --format html

# Watch mode
htms compile app.htms -o dist/ --watch

# Check syntax only
htms check app.htms

# With custom template (HTML mode)
htms compile app.htms -o dist/ --format html --template custom.html

# Split templates for lazy loading (HTML mode)
htms compile app.htms -o dist/ --format html --split-templates
```

---

## Common Patterns for AI Agents

### 1. List Rendering

```htms
// Use @for for single root element
div [class: "user-list"] @for(ctx.users as user) {
  div [class: "user-card"] {
    h3 {{ ${user.name} }}
    p {{ ${user.email} }}
  }
}

// Use @each for multiple siblings
@each ctx.users as user {
  h3 {{ ${user.name} }}
  p {{ ${user.email} }}
  hr
}
```

### 2. Conditional Rendering

```htms
// Show/hide element
div @if(ctx.showDetails) {
  p {{ Details here }}
}

// If/else (use block form)
@if ctx.isLoading {
  div {{ Loading... }}
} @else {
  div {{ Content }}
}

// Multiple conditions
@if ctx.status == "idle" {
  button [onClick: start] {{ Start }}
} @else if ctx.status == "running" {
  button [onClick: pause] {{ Pause }}
} @else {
  p {{ Finished }}
}
```

### 3. Forms

```htms
component LoginForm {
  form [onSubmit.prevent: handleLogin] {
    label {{ Email }}
    input [
      type: "email",
      bind: ctx.form.email,
      required: true
    ]

    label {{ Password }}
    input [
      type: "password",
      bind: ctx.form.password,
      required: true
    ]

    div @if(ctx.form.error) [class: "error"] {
      p {{ ${ctx.form.error} }}
    }

    button [
      type: "submit",
      disabled: ctx.form.isSubmitting
    ] {
      {{ ${ctx.form.isSubmitting ? "Logging in..." : "Login"} }}
    }
  }
}
```

**actions.ts:**
```typescript
export const actions = {
  handleLogin: async (ctx, event) => {
    ctx.data.form.isSubmitting = true;
    ctx.data.form.error = null;
    ctx.rerender();

    try {
      const response = await fetch('/api/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          email: ctx.data.form.email,
          password: ctx.data.form.password
        })
      });

      if (!response.ok) throw new Error('Login failed');

      const user = await response.json();
      ctx.data.user = user;
      ctx.data.isAuthenticated = true;
      window.location.hash = '#/dashboard';
    } catch (error) {
      ctx.data.form.error = error.message;
    } finally {
      ctx.data.form.isSubmitting = false;
      ctx.rerender();
    }
  }
};
```

### 4. Reusable Components

```htms
// Generic button
component Button(item: props) {
  button [
    class: props.variant == "primary" ? "btn-primary" : "btn-secondary",
    onClick: props.onClick,
    disabled: props.disabled
  ] {
    {{ ${props.label} }}
  }
}

// Usage
page home "/" {
  Button(item: ctx.submitButton)
  Button(item: ctx.cancelButton)
}
```

**Context:**
```typescript
setContext({
  submitButton: {
    label: 'Submit',
    variant: 'primary',
    onClick: 'handleSubmit',
    disabled: false
  },
  cancelButton: {
    label: 'Cancel',
    variant: 'secondary',
    onClick: 'handleCancel',
    disabled: false
  }
});
```

### 5. Navigation

```htms
component BottomNav {
  nav [class: "bottom-nav"] {
    a [
      href: "#/",
      class: ctx.route == "/" ? "active" : ""
    ] {
      i [class: "icon-home"]
      span {{ Home }}
    }

    a [
      href: "#/profile",
      class: ctx.route == "/profile" ? "active" : ""
    ] {
      i [class: "icon-user"]
      span {{ Profile }}
    }
  }
}
```

### 6. Empty States

```htms
div {
  @if ctx.items.length > 0 {
    ul @for(ctx.items as item) {
      li {{ ${item.name} }}
    }
  } @else {
    div [class: "empty-state"] {
      i [class: "icon-inbox"]
      p {{ No items found }}
      button [onClick: addFirstItem] {{ Add Item }}
    }
  }
}
```

### 7. Loading States

```htms
@if ctx.isLoading {
  div [class: "loading"] {
    span {{ Loading... }}
  }
} @else if ctx.error {
  div [class: "error"] {
    p {{ Error: ${ctx.error} }}
    button [onClick: retry] {{ Retry }}
  }
} @else {
  div @for(ctx.items as item) {
    p {{ ${item.name} }}
  }
}
```

---

## Critical Rules for AI Agents

### ✅ DO:

1. **Define components before use:**
   ```htms
   component NavBar { }  // Define first
   page home "/" {
     NavBar            // Then use
   }
   ```

2. **Use `{{ }}` for text (NO quotes inside):**
   ```htms
   h1 {{ Hello World }}  // ✅ Correct
   ```

3. **Use `${}` for variables:**
   ```htms
   p {{ Hello, ${ctx.user.name}! }}  // ✅ Correct
   ```

4. **Always call `ctx.rerender()` after state changes:**
   ```typescript
   ctx.data.count++;
   ctx.rerender();  // ✅ Required
   ```

5. **Export actions object:**
   ```typescript
   export const actions = { ... };  // ✅ Required
   ```

6. **Use PascalCase for component names:**
   ```htms
   component UserCard { }  // ✅ Correct
   ```

### ❌ DON'T:

1. **Quote text in `{{ }}`:**
   ```htms
   h1 {{ "Hello" }}  // ❌ Wrong
   ```

2. **Use component before defining:**
   ```htms
   page home "/" {
     NavBar  // ❌ Wrong if not defined above
   }
   ```

3. **Forget to rerender:**
   ```typescript
   ctx.data.count++;  // ❌ Missing ctx.rerender()
   ```

4. **Use wrong event handler format:**
   ```typescript
   const handleClick = () => { };  // ❌ Wrong
   export const actions = {
     handleClick: (ctx, event) => { }  // ✅ Correct
   };
   ```

---

## Complete Minimal Example

**app.htms:**
```htms
component NavBar {
  nav [class: "nav"] {
    a [href: "#/"] {{ Home }}
    a [href: "#/about"] {{ About }}
  }
}

page home "/" {
  NavBar
  main {
    h1 {{ Counter: ${ctx.count} }}
    button [onClick: increment] {{ + }}
    button [onClick: decrement] {{ - }}
  }
}

page about "/about" {
  NavBar
  main {
    h1 {{ About }}
    p {{ This is the about page }}
  }
}
```

**actions.ts:**
```typescript
export const actions = {
  increment: (ctx, event) => {
    ctx.data.count++;
    ctx.rerender();
  },

  decrement: (ctx, event) => {
    ctx.data.count--;
    ctx.rerender();
  }
};
```

**main.ts:**
```typescript
import { setContext, router } from './dist/router';
import { initEvents } from './dist/events';

setContext({ count: 0 });
initEvents();
router.init();
```

**runtime.ts:** (See "Required Files" section above)

**Compile:**
```bash
htms compile src/app.htms -o src/dist/
```

---

## Quick Decision Tree

**Need to render a list?**
- Single root element → Use `@for(ctx.items as item)`
- Multiple siblings → Use `@each ctx.items as item`

**Need conditional rendering?**
- Show/hide single element → Use `@if(condition)`
- Need else clause → Use `@if { } @else { }`
- Multiple conditions → Use `@if { } @else if { } @else { }`

**Need to handle events?**
- No parameters → `[onClick: handleClick]`
- With parameters → `[onClick: deleteItem(item.id)]`
- Prevent default → `[onSubmit.prevent: handleSubmit]`

**Need input binding?**
- Display only → `[value: ctx.form.email]`
- Two-way sync → `[bind: ctx.form.email]`

---

## Performance Tips

1. **Use element directives (`@for`, `@if`) over block directives** - They generate reusable functions
2. **Minimize `ctx.rerender()` calls** - Only call when state actually changes
3. **Use `bind` for inputs** - No need to manually rerender on input
4. **Debounce expensive operations:**
   ```typescript
   let timer;
   export const actions = {
     handleSearch: (ctx, event) => {
       clearTimeout(timer);
       timer = setTimeout(() => {
         // Search logic
         ctx.rerender();
       }, 300);
     }
   };
   ```

---

## Troubleshooting

**Component not found error?**
→ Define component before page that uses it

**Text not showing?**
→ Check you used `{{ }}` not quotes

**Event handler not working?**
→ Check it's exported in `actions` object

**UI not updating?**
→ Call `ctx.rerender()` after state changes

**Syntax error?**
→ Run `htms check app.htms` for details

---

## Summary

HTMS is:
- **Declarative** - Describe UI structure, not DOM operations
- **Safe** - Pure DOM API (no innerHTML)
- **Fast** - Compiles to vanilla JS, zero runtime
- **Simple** - HTML-like syntax with clear rules

**Basic workflow:**
1. Write `.htms` file with components and pages
2. Create `actions.ts` with event handlers
3. Create `main.ts` to initialize app
4. Compile: `htms compile app.htms -o dist/`
5. Run dev server (Vite, etc.)

**Remember:**
- Components first, pages second
- Text in `{{ }}`, variables in `{{ ${} }}`
- Always `ctx.rerender()` after state changes
- Export `actions` object

That's everything you need to build with HTMS!
