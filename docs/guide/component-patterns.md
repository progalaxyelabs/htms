# Component Patterns & Best Practices

Learn best practices and common patterns for building maintainable HTMS applications.

## Table of Contents

- [Component Design Principles](#component-design-principles)
- [Common Component Patterns](#common-component-patterns)
- [State Management](#state-management)
- [Form Handling](#form-handling)
- [List Rendering](#list-rendering)
- [Conditional Rendering](#conditional-rendering)
- [Component Composition](#component-composition)
- [Performance Tips](#performance-tips)
- [Code Organization](#code-organization)
- [Common Mistakes](#common-mistakes)

---

## Component Design Principles

### 1. Single Responsibility

Each component should do one thing well.

**❌ Bad:**
```htms
component UserDashboard {
  div {
    // Navigation
    nav {
      a [href: "#/"] {{ Home }}
      a [href: "#/profile"] {{ Profile }}
    }

    // Sidebar
    aside {
      ul {
        @each ctx.menuItems as item {
          li {{ ${item.name} }}
        }
      }
    }

    // Main content
    main {
      // User stats
      div { }
      // Recent activity
      div { }
      // Settings
      div { }
    }
  }
}
```

**✅ Good:**
```htms
component NavBar {
  nav {
    a [href: "#/"] {{ Home }}
    a [href: "#/profile"] {{ Profile }}
  }
}

component Sidebar {
  aside {
    ul {
      @each ctx.menuItems as item {
        li {{ ${item.name} }}
      }
    }
  }
}

component UserStats { /* ... */ }
component RecentActivity { /* ... */ }
component UserSettings { /* ... */ }

page dashboard "/" {
  NavBar
  div [class: "layout"] {
    Sidebar
    main {
      UserStats
      RecentActivity
      UserSettings
    }
  }
}
```

### 2. Reusability

Design components to be reused in different contexts.

**Example:**

```htms
// Generic button component
component Button(item: props) {
  button [
    class: props.variant == "primary" ? "btn btn-primary" : "btn btn-secondary",
    onClick: props.onClick,
    disabled: props.disabled
  ] {
    {{ ${props.label} }}
  }
}

// Usage in multiple contexts
page home "/" {
  Button(item: ctx.submitButton)
  Button(item: ctx.cancelButton)
  Button(item: ctx.deleteButton)
}
```

### 3. Prop Naming

Use clear, descriptive names for component parameters.

```htms
// ✅ Good
component UserCard(item: user) { }
component ProductCard(item: product) { }
component CommentItem(item: comment) { }

// ❌ Bad
component UserCard(item: data) { }
component ProductCard(item: x) { }
```

---

## Common Component Patterns

### Card Pattern

Reusable card component with slot:

```htms
component Card {
  div [class: "card"] {
    div [class: "card-body"] {
      @slot
    }
  }
}

component CardWithHeader(item: props) {
  div [class: "card"] {
    div [class: "card-header"] {
      h3 {{ ${props.title} }}
    }
    div [class: "card-body"] {
      @slot
    }
  }
}

// Usage
page home "/" {
  Card {
    p {{ Simple card content }}
  }

  CardWithHeader(item: ctx.card) {
    p {{ Card with header }}
  }
}
```

### List Item Pattern

Reusable list item with actions:

```htms
component TodoItem(item: todo) {
  li [class: todo.done ? "todo-item done" : "todo-item"] {
    input [
      type: "checkbox",
      checked: todo.done,
      onChange: toggleTodo(todo.id)
    ]

    span [class: "todo-text"] {
      {{ ${todo.text} }}
    }

    button [
      class: "btn-delete",
      onClick: deleteTodo(todo.id)
    ] {
      i [class: "icon-trash"]
    }
  }
}

page todos "/" {
  ul [class: "todo-list"] {
    @each ctx.todos as todo {
      TodoItem(item: todo)
    }
  }
}
```

### Modal Pattern

Modal/dialog component:

```htms
component Modal(item: props) {
  div [class: "modal-overlay"] @if(props.isOpen) {
    div [class: "modal-content"] {
      header [class: "modal-header"] {
        h2 {{ ${props.title} }}
        button [
          class: "btn-close",
          onClick: props.onClose
        ] {
          span {{ × }}
        }
      }

      div [class: "modal-body"] {
        @slot
      }

      footer [class: "modal-footer"] {
        button [onClick: props.onClose] {
          {{ Cancel }}
        }
        button [
          class: "btn-primary",
          onClick: props.onConfirm
        ] {
          {{ Confirm }}
        }
      }
    }
  }
}
```

### Navigation Pattern

Bottom navigation for mobile apps:

```htms
component BottomNav {
  nav [class: "bottom-nav"] {
    a [
      href: "#/",
      class: ctx.currentRoute == "/" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-home"]
      span {{ Home }}
    }

    a [
      href: "#/search",
      class: ctx.currentRoute == "/search" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-search"]
      span {{ Search }}
    }

    a [
      href: "#/profile",
      class: ctx.currentRoute == "/profile" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-user"]
      span {{ Profile }}
    }
  }
}
```

### Avatar Pattern

User avatar with fallback:

```htms
component Avatar(item: user) {
  div [class: "avatar"] {
    img @if(user.avatar) [
      src: user.avatar,
      alt: user.name
    ]

    div @if(!user.avatar) [
      class: "avatar-fallback"
    ] {
      span {{ ${user.name.charAt(0).toUpperCase()} }}
    }
  }
}
```

---

## State Management

### Local vs Global State

**Global State** (in `ctx`):
- User authentication
- Current route
- App-wide settings
- Shared data (cached API responses)

**Local State** (in component logic):
- Form input values (if not persisted)
- UI state (accordion open/closed)
- Temporary flags

### Updating State

Always use the `ctx.data` object and call `ctx.rerender()`:

```typescript
// actions.ts
export const actions = {
  // ✅ Good
  updateUserName: (newName: string) => (ctx, event) => {
    ctx.data.user.name = newName;
    ctx.rerender();
  },

  // ❌ Bad (creates new reference, doesn't rerender)
  badUpdate: (ctx, event) => {
    ctx.data = { ...ctx.data, user: { name: 'New Name' } };
    // Missing ctx.rerender()!
  }
};
```

### Derived State

Compute derived values in actions or before rendering:

```typescript
// In main.ts or actions
setContext({
  todos: [
    { id: 1, text: 'Buy milk', done: false },
    { id: 2, text: 'Write docs', done: true }
  ],
  // Derived state
  get activeTodos() {
    return this.todos.filter(t => !t.done);
  },
  get completedCount() {
    return this.todos.filter(t => t.done).length;
  }
});
```

```htms
page todos "/" {
  p {{ ${ctx.activeTodos.length} active }}
  p {{ ${ctx.completedCount} completed }}
}
```

### State Initialization

Initialize state in `main.ts`:

```typescript
import { setContext } from './dist/router';

// Default state
const initialState = {
  user: null,
  isAuthenticated: false,
  posts: [],
  ui: {
    sidebarOpen: false,
    theme: 'light'
  }
};

// Load from localStorage
const savedState = localStorage.getItem('appState');
const state = savedState ? JSON.parse(savedState) : initialState;

setContext(state);

// Save on updates
export const saveState = (ctx) => {
  localStorage.setItem('appState', JSON.stringify(ctx.data));
};
```

---

## Form Handling

### Simple Form

```htms
component LoginForm {
  form [onSubmit.prevent: handleLogin] {
    div [class: "form-group"] {
      label {{ Email }}
      input [
        type: "email",
        bind: ctx.form.email,
        required: true,
        placeholder: "you@example.com"
      ]
    }

    div [class: "form-group"] {
      label {{ Password }}
      input [
        type: "password",
        bind: ctx.form.password,
        required: true
      ]
    }

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

      // Redirect
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

### Form Validation

```htms
component SignupForm {
  form [onSubmit.prevent: handleSignup] {
    div [class: "form-group"] {
      label {{ Email }}
      input [
        type: "email",
        bind: ctx.form.email,
        onBlur: validateEmail
      ]
      span @if(ctx.form.errors.email) [class: "error"] {
        {{ ${ctx.form.errors.email} }}
      }
    }

    div [class: "form-group"] {
      label {{ Password }}
      input [
        type: "password",
        bind: ctx.form.password,
        onInput: validatePassword
      ]
      span @if(ctx.form.errors.password) [class: "error"] {
        {{ ${ctx.form.errors.password} }}
      }
    }

    button [
      type: "submit",
      disabled: ctx.form.hasErrors
    ] {
      {{ Sign Up }}
    }
  }
}
```

**actions.ts:**

```typescript
export const actions = {
  validateEmail: (ctx, event) => {
    const email = ctx.data.form.email;
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

    if (!emailRegex.test(email)) {
      ctx.data.form.errors.email = 'Invalid email address';
    } else {
      delete ctx.data.form.errors.email;
    }

    ctx.data.form.hasErrors = Object.keys(ctx.data.form.errors).length > 0;
    ctx.rerender();
  },

  validatePassword: (ctx, event) => {
    const password = ctx.data.form.password;

    if (password.length < 8) {
      ctx.data.form.errors.password = 'Password must be at least 8 characters';
    } else {
      delete ctx.data.form.errors.password;
    }

    ctx.data.form.hasErrors = Object.keys(ctx.data.form.errors).length > 0;
    ctx.rerender();
  },

  handleSignup: async (ctx, event) => {
    // Validate all fields
    actions.validateEmail(ctx, event);
    actions.validatePassword(ctx, event);

    if (ctx.data.form.hasErrors) return;

    // Submit form...
  }
};
```

---

## List Rendering

### Basic List

```htms
ul {
  @each ctx.items as item {
    li {{ ${item.name} }}
  }
}
```

### List with Components

```htms
component ListItem(item: data) {
  li [class: "list-item"] {
    span {{ ${data.name} }}
    button [onClick: removeItem(data.id)] {
      {{ Remove }}
    }
  }
}

page list "/" {
  ul {
    @each ctx.items as item {
      ListItem(item: item)
    }
  }
}
```

### Element Directive (Better Performance)

```htms
// Generates reusable function
ul @for(ctx.items as item) {
  li {
    span {{ ${item.name} }}
    button [onClick: removeItem(item.id)] {{ Remove }}
  }
}
```

### Empty State

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
      button [onClick: addFirstItem] {
        {{ Add Your First Item }}
      }
    }
  }
}
```

### Loading State

```htms
div {
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
    ul @for(ctx.items as item) {
      li {{ ${item.name} }}
    }
  }
}
```

### Infinite Scroll

```htms
div [class: "scroll-container", onScroll: handleScroll] {
  div @for(ctx.posts as post) {
    article [class: "post"] {
      h2 {{ ${post.title} }}
      p {{ ${post.excerpt} }}
    }
  }

  div @if(ctx.hasMore) [class: "loading-more"] {
    span {{ Loading more... }}
  }
}
```

**actions.ts:**

```typescript
export const actions = {
  handleScroll: (ctx, event) => {
    const el = event.target as HTMLElement;
    const bottom = el.scrollHeight - el.scrollTop === el.clientHeight;

    if (bottom && ctx.data.hasMore && !ctx.data.isLoadingMore) {
      actions.loadMore(ctx, event);
    }
  },

  loadMore: async (ctx, event) => {
    ctx.data.isLoadingMore = true;
    ctx.rerender();

    const nextPage = ctx.data.currentPage + 1;
    const response = await fetch(`/api/posts?page=${nextPage}`);
    const newPosts = await response.json();

    ctx.data.posts.push(...newPosts);
    ctx.data.currentPage = nextPage;
    ctx.data.hasMore = newPosts.length > 0;
    ctx.data.isLoadingMore = false;
    ctx.rerender();
  }
};
```

---

## Conditional Rendering

### Show/Hide

```htms
div @if(ctx.showDetails) {
  p {{ Detailed information here }}
}

button [onClick: toggleDetails] {
  {{ ${ctx.showDetails ? "Hide" : "Show"} Details }}
}
```

### Multiple Conditions

```htms
@if ctx.status == "idle" {
  button [onClick: start] {{ Start }}
} @else if ctx.status == "running" {
  button [onClick: pause] {{ Pause }}
  button [onClick: stop] {{ Stop }}
} @else if ctx.status == "paused" {
  button [onClick: resume] {{ Resume }}
  button [onClick: stop] {{ Stop }}
} @else {
  p {{ Finished }}
  button [onClick: reset] {{ Reset }}
}
```

### Conditional Classes

```htms
div [class: ctx.isActive ? "card active" : "card"] {
  p {{ Content }}
}

button [class: ctx.theme == "dark" ? "btn btn-dark" : "btn btn-light"] {
  {{ Toggle Theme }}
}
```

### Conditional Attributes

```htms
button [
  disabled: ctx.isSubmitting || !ctx.form.isValid,
  class: ctx.isPrimary ? "btn-primary" : "btn-secondary"
] {
  {{ Submit }}
}
```

---

## Component Composition

### Nested Components

```htms
component Icon(item: props) {
  i [class: props.name]
}

component Button(item: props) {
  button [class: "btn", onClick: props.onClick] {
    Icon(item: props.icon)
    span {{ ${props.label} }}
  }
}

component Toolbar {
  div [class: "toolbar"] {
    Button(item: ctx.saveButton)
    Button(item: ctx.cancelButton)
    Button(item: ctx.deleteButton)
  }
}
```

### Slots for Flexibility

```htms
component Panel {
  div [class: "panel"] {
    div [class: "panel-header"] {
      h2 {{ Panel Title }}
    }
    div [class: "panel-body"] {
      @slot
    }
  }
}

page home "/" {
  Panel {
    p {{ Custom content inside panel }}
    ul {
      li {{ Item 1 }}
      li {{ Item 2 }}
    }
  }
}
```

### Layout Components

```htms
component MainLayout {
  div [class: "app-container"] {
    NavBar
    div [class: "content-wrapper"] {
      Sidebar
      main [class: "main-content"] {
        @slot
      }
    }
    Footer
  }
}

page dashboard "/" {
  MainLayout {
    h1 {{ Dashboard }}
    UserStats
    RecentActivity
  }
}
```

---

## Performance Tips

### 1. Use Element Directives

Element directives (`@for`, `@if`) generate reusable functions:

```htms
// ✅ Better - generates reusable function
div @for(ctx.items as item) {
  span {{ ${item.name} }}
}

// ✓ OK - inline generation
@each ctx.items as item {
  div {
    span {{ ${item.name} }}
  }
}
```

### 2. Minimize Re-renders

Only call `ctx.rerender()` when necessary:

```typescript
// ❌ Bad - rerenders on every keystroke
export const actions = {
  handleInput: (ctx, event) => {
    ctx.data.searchQuery = event.target.value;
    ctx.rerender(); // Expensive!
  }
};

// ✅ Good - use binding for input, rerender on submit
```

```htms
input [bind: ctx.searchQuery]
button [onClick: performSearch] {{ Search }}
```

### 3. Debounce Expensive Operations

```typescript
let debounceTimer;

export const actions = {
  handleSearch: (ctx, event) => {
    clearTimeout(debounceTimer);

    debounceTimer = setTimeout(() => {
      // Perform search
      ctx.data.results = searchItems(ctx.data.searchQuery);
      ctx.rerender();
    }, 300);
  }
};
```

### 4. Lazy Load Images

```htms
img [
  src: item.thumbnail,
  loading: "lazy",
  alt: item.title
]
```

### 5. Virtual Scrolling for Large Lists

For lists with 1000+ items, consider implementing virtual scrolling in your actions.

---

## Code Organization

### File Structure

```
src/
├── app.htms              # Main HTMS file
├── components/
│   ├── common.htms       # Shared components (Button, Input, etc.)
│   ├── layout.htms       # Layout components
│   └── features.htms     # Feature-specific components
├── pages/
│   ├── home.htms
│   ├── profile.htms
│   └── settings.htms
├── main.ts               # App initialization
├── actions.ts            # Event handlers
├── api.ts                # API calls
└── utils.ts              # Helper functions
```

### Multiple HTMS Files

Split large apps into multiple files:

**components/common.htms:**
```htms
component Button(item: props) { }
component Input(item: props) { }
component Card { }
```

**pages/home.htms:**
```htms
page home "/" {
  NavBar
  HeroSection
}
```

Compile separately:
```bash
htms compile src/components/common.htms -o dist/components/
htms compile src/pages/home.htms -o dist/pages/
```

---

## Common Mistakes

### 1. Forgetting to Rerender

```typescript
// ❌ State updated but UI not refreshed
export const actions = {
  updateName: (ctx, event) => {
    ctx.data.user.name = 'New Name';
    // Missing ctx.rerender()!
  }
};

// ✅ Always call rerender after state changes
export const actions = {
  updateName: (ctx, event) => {
    ctx.data.user.name = 'New Name';
    ctx.rerender();
  }
};
```

### 2. Using Wrong Quote Style

```htms
// ❌ Wrong - don't quote text in {{ }}
h1 {{ "Hello World" }}

// ✅ Correct
h1 {{ Hello World }}
```

### 3. Component Reference Before Definition

```htms
// ❌ Wrong - UserCard not defined yet
page home "/" {
  UserCard
}

component UserCard { }

// ✅ Correct - define first
component UserCard { }

page home "/" {
  UserCard
}
```

### 4. Missing Action Export

```htms
button [onClick: handleClick] {{ Click }}
```

```typescript
// ❌ Not exported
const handleClick = (ctx, event) => { };

// ✅ Exported in actions object
export const actions = {
  handleClick: (ctx, event) => { }
};
```

### 5. Mutating Context Directly Without Rerender

```typescript
// ❌ Mutation without rerender
ctx.data.count++;

// ✅ Mutate and rerender
ctx.data.count++;
ctx.rerender();
```

---

## Next Steps

- [Examples](/examples/) - Real-world applications
- [API Reference](/api/generated-code) - Understanding generated code
- [Build & Deployment](/guide/build-deploy) - Production tips
