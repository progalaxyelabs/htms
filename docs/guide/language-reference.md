# Language Reference

Complete syntax reference for HTMS (HTM Script).

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Components](#components)
- [Sections](#sections)
- [Pages](#pages)
- [Elements](#elements)
- [Attributes](#attributes)
- [Text Content](#text-content)
- [Control Flow](#control-flow)
- [Element Directives](#element-directives)
- [Data Binding](#data-binding)
- [Event Handling](#event-handling)
- [Comments](#comments)
- [Context (ctx)](#context-ctx)

---

## Basic Syntax

HTMS uses a clear, intuitive syntax:

- **Square brackets `[]`** for attributes
- **Curly braces `{}`** for children/body
- **Double braces `{{ }}`** for text content
- **`@` prefix** for directives
- **PascalCase** for component references

### Minimal Example

```htms
component HelloWorld {
  div [class: "greeting"] {
    h1 {{ Hello, World! }}
  }
}
```

---

## Components

Components are reusable UI building blocks.

### Basic Component

```htms
component ComponentName {
  // Element structure
}
```

**Example:**

```htms
component Button {
  button [class: "btn"] {
    {{ Click Me }}
  }
}
```

### Component with Parameters

Pass data to components using the `item` parameter:

```htms
component ComponentName(item: paramName) {
  // Use paramName.property
}
```

**Example:**

```htms
component UserCard(item: user) {
  div [class: "card"] {
    img [src: user.avatar]
    h3 {{ ${user.name} }}
    p {{ ${user.email} }}
  }
}
```

### Component with Slot

Use `@slot` to allow children to be passed in:

```htms
component Card {
  div [class: "card"] {
    div [class: "card-header"] {
      {{ Card Title }}
    }
    div [class: "card-body"] {
      @slot
    }
  }
}
```

**Usage:**

```htms
page home "/" {
  Card {
    p {{ This content fills the slot }}
  }
}
```

### Component Usage

Reference components by name (no quotes, PascalCase):

```htms
page home "/" {
  NavBar
  UserCard(item: ctx.currentUser)
  Card {
    p {{ Slot content }}
  }
}
```

---

## Sections

Sections are reusable page fragments (like components, but semantically for page sections).

```htms
section SectionName {
  // Element structure
}
```

**Example:**

```htms
section HeroSection {
  div [class: "hero"] {
    h1 {{ ${ctx.hero.title} }}
    p {{ ${ctx.hero.subtitle} }}
    button [onClick: getStarted] {
      {{ Get Started }}
    }
  }
}
```

**Usage:**

```htms
page home "/" {
  HeroSection
}
```

---

## Pages

Pages define routes in your application.

```htms
page pageName "route-path" {
  // Components and elements
}
```

### Basic Page

```htms
page home "/" {
  h1 {{ Home Page }}
}
```

### Page with Route Parameters

```htms
page userProfile "/users/:id" {
  div {
    h1 {{ User Profile }}
    // Access route param via ctx.params.id
  }
}
```

### Multiple Pages

```htms
page home "/" {
  NavBar
  main {{ Welcome }}
}

page about "/about" {
  NavBar
  main {{ About Us }}
}

page contact "/contact" {
  NavBar
  ContactForm
}

page notFound "/404" {
  h1 {{ 404 - Not Found }}
}
```

---

## Elements

Elements are HTML tags. Any valid HTML tag can be used.

### Element Syntax

```htms
tagname
tagname [attributes]
tagname { children }
tagname [attributes] { children }
```

### Common Elements

```htms
// Structural
div
span
section
article
header
footer
nav
main
aside

// Text
h1, h2, h3, h4, h5, h6
p
strong
em
small
code
pre

// Forms
form
input
textarea
select
option
button
label

// Lists
ul
ol
li

// Media
img
video
audio
iframe

// Other
a
table
tr
td
th
canvas
svg
```

### Self-Closing Elements

Elements without children (no `{}` block):

```htms
img [src: "/logo.png", alt: "Logo"]
input [type: "text", name: "email"]
hr
br
```

### Nested Elements

```htms
div [class: "container"] {
  header {
    h1 {{ Title }}
  }
  main {
    section {
      p {{ Content }}
    }
  }
  footer {
    p {{ Footer }}
  }
}
```

---

## Attributes

Attributes are defined in square brackets with `key: value` syntax.

### Basic Attributes

```htms
element [key: value]
element [key1: value1, key2: value2]
```

**Example:**

```htms
div [class: "container", id: "main"]
a [href: "#/about", target: "_blank"]
img [src: "/logo.png", alt: "Logo", width: "100"]
```

### Multi-line Attributes

For readability:

```htms
input [
  type: "email",
  name: "user_email",
  placeholder: "you@example.com",
  required: true,
  class: "form-control"
]
```

### Dynamic Attributes

Use context variables:

```htms
div [class: ctx.isActive ? "active" : "inactive"]
img [src: ctx.user.avatar]
a [href: ctx.link.url]
```

### Boolean Attributes

```htms
input [type: "checkbox", checked: true]
input [type: "text", disabled: ctx.isLoading]
button [disabled: ctx.form.isSubmitting]
```

### Data Attributes

```htms
div [data-id: "123", data-type: "user"]
button [data-testid: "submit-btn"]
```

### Common Attributes

- `class` - CSS classes
- `id` - Element ID
- `style` - Inline styles (string)
- `href` - Link URL (for `<a>`)
- `src` - Source URL (for `<img>`, `<script>`, etc.)
- `alt` - Alt text (for `<img>`)
- `type` - Input type
- `name` - Form field name
- `value` - Input value
- `placeholder` - Placeholder text
- `disabled` - Disable element
- `required` - Required field
- `checked` - Checkbox checked
- `target` - Link target (`_blank`, etc.)

---

## Text Content

Text nodes use double braces `{{ }}`.

### Static Text

```htms
h1 {{ Hello, World! }}
p {{ This is a paragraph of text }}
```

No quotes needed inside `{{ }}`:

```htms
// ✅ Correct
p {{ He said "Hello" and she replied 'Hi!' }}

// ❌ Wrong (don't use quotes)
p {{ "Hello" }}
```

### Dynamic Text (Interpolation)

Use `${}` for variables:

```htms
h1 {{ ${ctx.title} }}
p {{ Hello, ${ctx.user.name}! }}
span {{ You have ${ctx.notifications.length} notifications }}
```

### Mixed Static and Dynamic

```htms
p {
  {{ Welcome back, }}
  strong {{ ${ctx.user.name} }}
  {{ ! Your last login was ${ctx.lastLogin}. }}
}
```

### Text Shorthand

For elements with only text content, you can omit braces:

```htms
// Verbose
h1 {
  {{ Welcome }}
}

// Shorthand (recommended)
h1 {{ Welcome }}
```

### Multi-line Text

```htms
p {{
  This is a very long paragraph that spans
  multiple lines. It will be rendered as
  a single text node.
}}

pre {{
  function example() {
    console.log("Code block");
  }
}}
```

---

## Control Flow

### Conditional Rendering (`@if` / `@else`)

#### Block Form

```htms
@if condition {
  // Elements to render if true
}

@if condition {
  // Elements if true
} @else {
  // Elements if false
}

@if condition1 {
  // Elements if condition1
} @else if condition2 {
  // Elements if condition2
} @else {
  // Elements otherwise
}
```

**Examples:**

```htms
@if ctx.isLoggedIn {
  div {{ Welcome, ${ctx.user.name}! }}
} @else {
  a [href: "#/login"] {{ Sign In }}
}

@if ctx.status == "loading" {
  div {{ Loading... }}
} @else if ctx.status == "error" {
  div {{ Error: ${ctx.error} }}
} @else {
  div {{ Data loaded successfully }}
}
```

### Loop Iteration (`@each`)

#### Block Form

```htms
@each collection as item {
  // Elements for each item
}

@each collection as item, index {
  // Elements with index
}
```

**Examples:**

```htms
ul {
  @each ctx.items as item {
    li {{ ${item.name} }}
  }
}

ul {
  @each ctx.users as user, index {
    li {
      span {{ ${index + 1}. }}
      strong {{ ${user.name} }}
    }
  }
}
```

### Nested Control Flow

```htms
@each ctx.categories as category {
  div [class: "category"] {
    h2 {{ ${category.name} }}

    @if category.items.length > 0 {
      ul {
        @each category.items as item {
          li {{ ${item.name} }}
        }
      }
    } @else {
      p {{ No items in this category }}
    }
  }
}
```

---

## Element Directives

Element directives are special attributes that generate optimized reusable functions.

### List Rendering (`@for`)

Render an element multiple times from an array:

```htms
element @for(collection as item) {
  // Use item here
}

element @for(collection as item, index) {
  // Use item and index
}
```

**Generated Code:**
The compiler generates a separate function that returns `HTMLElement[]`.

**Examples:**

```htms
// Simple list
div [class: "user-list"] @for(ctx.users as user) {
  div [class: "user-card"] {
    h3 {{ ${user.name} }}
    p {{ ${user.email} }}
  }
}

// With index
ul @for(ctx.items as item, index) {
  li {
    span [class: "index"] {{ ${index + 1} }}
    span {{ ${item.text} }}
  }
}

// Nested @for
div @for(ctx.categories as category) {
  h2 {{ ${category.name} }}
  ul @for(category.items as item) {
    li {{ ${item.name} }}
  }
}
```

### Conditional Rendering (`@if`)

Render an element only if a condition is true:

```htms
element @if(condition) {
  // Children
}
```

**Generated Code:**
The compiler generates a function that returns `HTMLElement | null`.

**Examples:**

```htms
div [class: "error-message"] @if(ctx.hasError) {
  p {{ ${ctx.errorMessage} }}
}

button [class: "delete-btn"] @if(ctx.user.isAdmin) {
  span {{ Delete }}
}

// With nested content
div @if(ctx.showDetails) {
  h3 {{ Details }}
  p {{ ${ctx.details.description} }}
  button [onClick: hideDetails] {{ Hide }}
}
```

### Combining Directives

You can use `@if` and `@for` together:

```htms
// Show list only if items exist
div @if(ctx.items.length > 0) {
  ul @for(ctx.items as item) {
    li {{ ${item.name} }}
  }
}

// Conditional items in list
ul @for(ctx.posts as post) {
  li {
    h3 {{ ${post.title} }}
    span @if(post.isPinned) {
      i [class: "icon-pin"]
    }
  }
}
```

### Element Directive vs Block Directive

**Element Directive** (generates reusable function):
```htms
div @for(ctx.items as item) {
  span {{ ${item.name} }}
}
```

**Block Directive** (inline generation):
```htms
@each ctx.items as item {
  div {
    span {{ ${item.name} }}
  }
}
```

**When to use which:**
- Use **element directives** (`@for`, `@if`) when you want a single root element
- Use **block directives** (`@each`, `@if/@else`) when you need multiple siblings or else clauses
- Element directives generate reusable functions (better for performance if reused)

---

## Data Binding

### One-way Binding

Display data from context:

```htms
input [type: "text", value: ctx.form.name]
textarea [value: ctx.form.message]
```

The value is set once when the element is created.

### Two-way Binding

Automatically sync input value with context:

```htms
input [type: "text", bind: ctx.form.name]
textarea [bind: ctx.form.message]
```

**How it works:**
- `bind` generates both `value` attribute and `onInput` handler
- Changes to the input automatically update `ctx.form.name`
- Calling `ctx.rerender()` updates the input from context

**Supported inputs:**
- `<input type="text">`
- `<input type="email">`
- `<input type="password">`
- `<input type="number">`
- `<input type="checkbox">` (binds to boolean)
- `<textarea>`

**Example:**

```htms
form [onSubmit: handleSubmit] {
  label {{ Name }}
  input [type: "text", bind: ctx.form.name]

  label {{ Email }}
  input [type: "email", bind: ctx.form.email]

  label {{ Message }}
  textarea [bind: ctx.form.message, rows: "5"]

  label {
    input [type: "checkbox", bind: ctx.form.subscribe]
    span {{ Subscribe to newsletter }}
  }

  button {{ Submit }}
}
```

---

## Event Handling

### Event Attributes

Events start with `on` prefix:

```htms
element [onEventName: handlerName]
```

**Common events:**
- `onClick` - Click event
- `onSubmit` - Form submit
- `onInput` - Input change (text inputs)
- `onChange` - Change event (checkboxes, selects)
- `onFocus` - Focus event
- `onBlur` - Blur event
- `onKeydown` - Key down
- `onKeyup` - Key up
- `onMouseenter` - Mouse enter
- `onMouseleave` - Mouse leave

### Basic Event Handler

```htms
button [onClick: handleClick] {
  {{ Click Me }}
}
```

**Corresponding actions.ts:**

```typescript
export const actions = {
  handleClick: (ctx, event) => {
    console.log('Clicked!', ctx.data);
    ctx.rerender();
  }
};
```

### Event Handler with Parameters

Pass arguments to handlers:

```htms
button [onClick: deleteItem(item.id)] {
  {{ Delete }}
}

@each ctx.items as item {
  button [onClick: selectItem(item.id)] {
    {{ Select ${item.name} }}
  }
}
```

**Corresponding actions.ts:**

```typescript
export const actions = {
  deleteItem: (id: number) => (ctx, event) => {
    ctx.data.items = ctx.data.items.filter(item => item.id !== id);
    ctx.rerender();
  },

  selectItem: (id: number) => (ctx, event) => {
    ctx.data.selectedId = id;
    ctx.rerender();
  }
};
```

### Event Modifiers

Modify event behavior with dot notation:

```htms
element [onEvent.modifier: handler]
```

**Available modifiers:**

- `.prevent` - Call `event.preventDefault()`
- `.stop` - Call `event.stopPropagation()`
- `.once` - Remove handler after first trigger

**Examples:**

```htms
// Prevent form submission default
form [onSubmit.prevent: handleSubmit] {
  button {{ Submit }}
}

// Stop propagation
div [onClick: outer] {
  button [onClick.stop: inner] {
    {{ Click (won't bubble) }}
  }
}

// Run only once
button [onClick.once: showWelcome] {
  {{ Show Welcome Message }}
}

// Multiple modifiers
a [onClick.prevent.stop: navigate] {
  {{ Link }}
}
```

### Form Events

```htms
form [onSubmit.prevent: handleSubmit] {
  input [
    type: "text",
    bind: ctx.form.email,
    onFocus: trackFocus,
    onBlur: validateField
  ]

  button {{ Submit }}
}
```

### Event Handler Context

All event handlers receive two arguments:

```typescript
(ctx: ActionContext, event: Event) => void
```

**ActionContext:**
```typescript
{
  data: Record<string, unknown>,  // Your app state
  rerender: () => void             // Function to re-render
}
```

**Example:**

```typescript
export const actions = {
  handleSubmit: (ctx, event) => {
    // Access state
    console.log(ctx.data.form);

    // Modify state
    ctx.data.form.submitted = true;

    // Re-render UI
    ctx.rerender();
  }
};
```

---

## Comments

### Single-line Comments

```htms
// This is a single-line comment
component NavBar {
  // This component renders the navigation
  nav [class: "navbar"] {
    a [href: "#/"] {{ Home }}
  }
}
```

### Multi-line Comments

```htms
/*
  This is a multi-line comment
  It can span multiple lines
  Useful for documentation
*/

component Card {
  /* Card header */
  div [class: "card-header"] {
    h3 {{ Title }}
  }

  /* Card body */
  div [class: "card-body"] {
    @slot
  }
}
```

---

## Context (ctx)

The `ctx` object is your application state, passed to all pages and components.

### Setting Context

In your `main.ts`:

```typescript
import { setContext } from './dist/router';

setContext({
  user: {
    id: 1,
    name: 'John Doe',
    avatar: '/avatar.jpg',
    isAdmin: false
  },
  posts: [],
  form: {
    email: '',
    message: ''
  },
  isLoading: false,
  error: null
});
```

### Using Context

Access nested properties with dot notation:

```htms
page home "/" {
  // Direct property
  h1 {{ ${ctx.user.name} }}

  // Nested property
  img [src: ctx.user.avatar]

  // In conditions
  @if ctx.user.isAdmin {
    button {{ Admin Panel }}
  }

  // In loops
  @each ctx.posts as post {
    div {{ ${post.title} }}
  }

  // In expressions
  div [class: ctx.isLoading ? "loading" : "loaded"]
}
```

### Updating Context

In event handlers:

```typescript
export const actions = {
  updateUser: (ctx, event) => {
    // Modify context
    ctx.data.user.name = 'Jane Doe';

    // Re-render to reflect changes
    ctx.rerender();
  },

  addPost: (ctx, event) => {
    ctx.data.posts.push({
      id: Date.now(),
      title: 'New Post'
    });
    ctx.rerender();
  }
};
```

### Context Type

By default, context is typed as `Record<string, unknown>`. You can provide better types in your `actions.ts`:

```typescript
interface AppContext {
  user: {
    id: number;
    name: string;
    avatar: string;
    isAdmin: boolean;
  };
  posts: Array<{
    id: number;
    title: string;
    content: string;
  }>;
  form: {
    email: string;
    message: string;
  };
  isLoading: boolean;
  error: string | null;
}

export const actions = {
  myHandler: (ctx: { data: AppContext, rerender: () => void }, event: Event) => {
    // Now ctx.data is fully typed!
    ctx.data.user.name; // TypeScript knows this is a string
  }
};
```

---

## Syntax Summary Table

| Feature | Syntax | Example |
|---------|--------|---------|
| **Component** | `component Name { }` | `component NavBar { }` |
| **Section** | `section Name { }` | `section Hero { }` |
| **Page** | `page name "route" { }` | `page home "/" { }` |
| **Element** | `tagname` | `div`, `button` |
| **Attributes** | `[key: value]` | `[class: "foo"]` |
| **Children** | `{ ... }` | `div { span { } }` |
| **Text** | `{{ text }}` | `{{ Hello }}` |
| **Interpolation** | `{{ ${var} }}` | `{{ ${ctx.name} }}` |
| **Component ref** | `PascalCase` | `NavBar` |
| **Component param** | `(item: val)` | `Card(item: user)` |
| **Slot** | `@slot` | `@slot` |
| **If/else** | `@if cond { } @else { }` | `@if ctx.show { }` |
| **Each** | `@each arr as item { }` | `@each ctx.items as item { }` |
| **Element @for** | `el @for(arr as item) { }` | `div @for(ctx.users as u) { }` |
| **Element @if** | `el @if(cond) { }` | `div @if(ctx.show) { }` |
| **Binding** | `bind: ctx.path` | `bind: ctx.form.name` |
| **Event** | `onEvent: action` | `onClick: submit` |
| **Event param** | `onEvent: action(arg)` | `onClick: select(id)` |
| **Event modifier** | `onEvent.mod: action` | `onClick.prevent: nav` |
| **Comment** | `//` or `/* */` | `// comment` |

---

## Next Steps

- [Component Patterns](/guide/component-patterns) - Best practices
- [Examples](/examples/) - Real-world applications
- [API Reference](/api/generated-code) - Understanding generated code
