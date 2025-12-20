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
      text: View on GitHub
      link: https://github.com/progalaxyelabs/htms

features:
  - icon: ⚡
    title: Clean Syntax
    details: Intuitive syntax that combines the best of HTML, CSS, and JS conventions
  - icon: 🔒
    title: Compile-time Safety
    details: Component references validated at compile time - no magic strings
  - icon: 🔄
    title: Two-way Binding
    details: Built-in support for form input binding with automatic sync
  - icon: 🛠️
    title: Great Tooling
    details: VSCode extension with syntax highlighting, autocomplete, and diagnostics
  - icon: 📦
    title: Zero Runtime
    details: Compiles to standard Handlebars templates + vanilla JavaScript
  - icon: 🎯
    title: Framework Agnostic
    details: Works with any backend - just generate templates and serve
---

## Quick Example

```htms
component NavBar {
  nav [class: "navbar"] {
    a [href: "#/"] { {{ Home }} }
    a [href: "#/about"] { {{ About }} }
  }
}

page home "/" {
  NavBar
  main {
    h1 { ctx.title }
    p { {{ Welcome to HTMS! }} }
  }
}
```
