# HTMS Language Support for Visual Studio Code

Official VSCode extension for HTMS (HTM Script) language support.

## Features

### Syntax Highlighting
- Full syntax highlighting for HTMS files
- Support for components, sections, pages
- Highlighting for directives (`@if`, `@each`, `@slot`, `@else`)
- Expression highlighting in text content and attributes

### Code Completion
- Auto-completion for HTMS keywords (`component`, `section`, `page`)
- Auto-completion for directives (`@if`, `@each`, `@slot`, `@else`)
- HTML element suggestions
- Common attribute suggestions

### Diagnostics
- Real-time error and warning detection
- Powered by the HTMS compiler
- Clear error messages with location information

### Code Snippets
- `component` - Create a new component
- `section` - Create a new section
- `page` - Create a new page
- `@if` - Conditional rendering
- `@ifelse` - If-else conditional
- `@each` - Loop over items
- `@eachi` - Loop with index
- `button` - Button with click handler
- `input` - Input with two-way binding
- And more...

### Editor Features
- Auto-closing pairs for `{}`, `[]`, `()`, `{{}}`, `""`
- Smart indentation
- Comment support (`//` and `/* */`)
- Code folding with `#region` markers

## Installation

### From VSIX
1. Download the `.vsix` file
2. Open VSCode
3. Go to Extensions view (`Ctrl+Shift+X`)
4. Click the `...` menu at the top
5. Select "Install from VSIX..."
6. Choose the downloaded `.vsix` file

### From Source
1. Clone the repository
2. Navigate to `htms-vscode/`
3. Run `npm install`
4. Run `npm run compile`
5. Press `F5` to launch extension development host

## Usage

1. Create a new file with `.htms` extension
2. Start writing HTMS code
3. Enjoy syntax highlighting and auto-completion
4. Errors and warnings will appear in the Problems panel

## Example

```htms
component Button(text: string, onClick: function) {
  button [onClick: onClick] {
    {{ text }}
  }
}

page home "/" {
  div [class: "container"] {
    Button(text: "Click Me", onClick: handleClick)
  }
}
```

## Requirements

- VSCode version 1.80.0 or higher

## Extension Settings

Currently, this extension does not contribute any settings.

## Known Issues

- Go-to-definition not yet implemented
- Hover information not yet implemented

## Release Notes

### 0.1.0

Initial release:
- Syntax highlighting
- Code completion
- Real-time diagnostics
- Code snippets
- Auto-closing pairs
- Smart indentation

## Contributing

This extension is part of the HTMS project. Contributions are welcome!

## License

MIT
