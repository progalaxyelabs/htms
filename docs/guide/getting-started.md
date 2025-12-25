# Getting Started with HTMS

## What is HTMS?

HTMS (HTM Script) is a declarative language for building web user interfaces that compiles to **TypeScript with pure DOM API calls** or **static HTML files**. It provides compile-time safety, component composition, and a clean syntax that's easy to read and write.

### Key Features

- **Pure DOM API**: No `innerHTML`, no template strings - generates safe `document.createElement()` calls
- **XSS-Safe by Default**: Zero injection vulnerabilities
- **Compile-time Validation**: Component references validated before runtime
- **Dual Output Modes**: TypeScript (for dynamic apps) or HTML (for static sites)
- **Zero Runtime Overhead**: Compiles to vanilla JavaScript
- **Framework Agnostic**: Works with any backend or frontend setup

## Installation

### Install the Compiler

```bash
npm install --save-dev @progalaxyelabs/htms-compiler
```

### Install the CLI (Recommended)

```bash
npm install --save-dev @progalaxyelabs/htms-cli

# Or install globally
npm install -g @progalaxyelabs/htms-cli
```

### Install VSCode Extension (Optional)

Search for "HTMS" in the VSCode extensions marketplace, or install from the command line:

```bash
code --install-extension progalaxyelabs.htms-vscode
```

## Your First HTMS File

Create a file named `app.htms`:

```htms
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] {{ Home }}
    a [href: "#/about"] {{ About }}
  }
}

component Button {
  button [class: "btn btn-primary", onClick: handleClick] {
    {{ Click Me }}
  }
}

page home "/" {
  NavBar
  main [class: "container"] {
    h1 {{ Welcome to HTMS! }}
    p {{ This is a declarative UI language }}
    Button
  }
}

page about "/about" {
  NavBar
  main [class: "container"] {
    h1 {{ About }}
    p {{ HTMS compiles to pure DOM API calls }}
  }
}
```

## Compile Your First App

### Option 1: TypeScript Output (Default)

Generate TypeScript files for dynamic applications:

```bash
htms compile app.htms -o dist/
```

This generates:
- `dist/templates.ts` - Component render functions
- `dist/router.ts` - Hash-based router
- `dist/events.ts` - Event delegation system

### Option 2: HTML Output

Generate a single HTML file for static sites:

```bash
htms compile app.htms -o dist/ --format html
```

This generates:
- `dist/index.html` - Complete HTML with client-side routing

## Using the Generated Code

### TypeScript Output

Create `src/main.ts`:

```typescript
import { router } from './dist/router';
import { initEvents } from './dist/events';

// Set initial context (your app state)
import { setContext } from './dist/router';

setContext({
  user: {
    name: 'John Doe',
    avatar: '/avatar.jpg'
  },
  posts: [
    { id: 1, title: 'First Post', content: 'Hello World' }
  ]
});

// Initialize event handling
initEvents();

// Start router
router.init();
```

Create `src/actions.ts` (for event handlers):

```typescript
export const actions = {
  handleClick: (ctx, event) => {
    console.log('Button clicked!', ctx.data);
    ctx.rerender(); // Re-render the page
  },

  submitForm: (ctx, event) => {
    event.preventDefault();
    // Handle form submission
    ctx.data.formSubmitted = true;
    ctx.rerender();
  }
};
```

Create `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>HTMS App</title>
  <link rel="stylesheet" href="styles.css">
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

### HTML Output

The generated `dist/index.html` is ready to deploy! Just open it in a browser or host it on any static hosting service (Netlify, Vercel, GitHub Pages, etc.).

## Project Structure

Recommended structure for HTMS projects:

```
my-htms-app/
├── src/
│   ├── app.htms          # Your HTMS source
│   ├── main.ts           # App initialization
│   ├── actions.ts        # Event handlers
│   └── styles.css        # Your CSS
├── dist/                 # Generated files
│   ├── templates.ts
│   ├── router.ts
│   └── events.ts
├── index.html            # HTML entry point
├── package.json
└── vite.config.ts        # Or webpack config
```

## Using with Build Tools

### Vite

Install the Vite plugin:

```bash
npm install --save-dev @progalaxyelabs/htms-cli
```

Configure `vite.config.ts`:

```typescript
import { defineConfig } from 'vite';
import { htmsPlugin } from '@progalaxyelabs/htms-cli/vite-plugin';

export default defineConfig({
  plugins: [htmsPlugin()]
});
```

### Webpack

Use the CLI in watch mode:

```json
{
  "scripts": {
    "watch:htms": "htms compile src/app.htms -o dist/ --watch",
    "dev": "npm run watch:htms & webpack serve"
  }
}
```

## CLI Commands Reference

### compile

Compile HTMS files to TypeScript or HTML:

```bash
htms compile <input> -o <output-dir> [options]
```

**Options:**
- `-o, --output <dir>` - Output directory (required)
- `--format <type>` - Output format: `typescript` (default) or `html`
- `--template <file>` - Custom HTML template (HTML mode only)
- `--split-templates` - Generate separate template files (HTML mode only)
- `--watch` - Watch for file changes

**Examples:**

```bash
# Compile to TypeScript
htms compile app.htms -o dist/

# Compile to HTML
htms compile app.htms -o dist/ --format html

# Watch mode
htms compile app.htms -o dist/ --watch

# Custom HTML template
htms compile app.htms -o dist/ --format html --template custom.html

# Split templates for lazy loading
htms compile app.htms -o dist/ --format html --split-templates
```

### check

Check HTMS syntax without generating files:

```bash
htms check <input>
```

**Example:**

```bash
htms check app.htms
```

This validates:
- Syntax errors
- Undefined component references
- Duplicate declarations
- Invalid route formats
- Duplicate routes

### init (Coming Soon)

Initialize a new HTMS project:

```bash
htms init my-app
```

## Next Steps

Now that you have HTMS set up, learn about:

1. [Language Reference](/guide/language-reference) - Complete syntax guide
2. [Component Patterns](/guide/component-patterns) - Best practices for components
3. [Examples](/examples/todo-app) - Real-world examples
4. [API Reference](/api/generated-code) - Understanding generated code

## Quick Tips

1. **Component Names**: Use PascalCase (e.g., `NavBar`, `UserCard`)
2. **Text Nodes**: Use `{{ }}` for static text, `{{ ${var} }}` for dynamic
3. **Attributes**: Use `[key: value]` syntax
4. **Events**: Prefix with `on` (e.g., `onClick`, `onSubmit`)
5. **Validation**: Run `htms check` frequently during development
6. **Watch Mode**: Use `--watch` for rapid development

## Common Gotchas

### Text Shorthand

You can omit the braces for text-only children:

```htms
// Verbose
button [class: "btn"] {
  {{ Submit }}
}

// Shorthand (recommended)
button [class: "btn"] {{ Submit }}
```

### Component References

Component names must be defined before use:

```htms
// ❌ Wrong - UserCard not defined yet
page home "/" {
  UserCard
}

component UserCard {
  div {{ User }}
}

// ✅ Correct - Define first
component UserCard {
  div {{ User }}
}

page home "/" {
  UserCard
}
```

### Event Handlers

Event handler names in HTMS must match exported functions in `actions.ts`:

```htms
// In app.htms
button [onClick: handleSubmit] {{ Submit }}
```

```typescript
// In actions.ts
export const actions = {
  handleSubmit: (ctx, event) => {
    // Your logic here
  }
};
```

## Troubleshooting

### Compilation Errors

If you see compilation errors:

1. Run `htms check app.htms` to see detailed diagnostics
2. Check that all component names are defined before use
3. Verify attribute syntax uses `[key: value]` format
4. Ensure text nodes are wrapped in `{{ }}`

### Runtime Errors

If the app doesn't work at runtime:

1. Check browser console for JavaScript errors
2. Verify `actions.ts` exports all referenced handlers
3. Ensure `#app` element exists in your HTML
4. Check that context data matches what templates expect

### VSCode Not Highlighting

1. Install the HTMS VSCode extension
2. Reload VSCode
3. Check file extension is `.htms`
4. Try opening a different file and coming back

## Getting Help

- [GitHub Issues](https://github.com/progalaxyelabs/htms/issues)
- [Documentation](/guide/language-reference)
- [Examples](/examples/)

## What's Next?

Continue to the [Language Reference](/guide/language-reference) to learn the complete HTMS syntax, or jump to [Examples](/examples/) to see real-world applications.
