# HTMS CLI

Command-line interface for the HTMS compiler.

## Installation

```bash
npm install -g @htms/cli
```

## Usage

### Compile

Compile a `.htms` file to TypeScript:

```bash
htms compile src/app.htms -o dist/
```

Options:
- `-o, --output <dir>` - Output directory (default: `dist`)
- `-w, --watch` - Watch for changes
- `-q, --quiet` - Suppress output

### Check

Validate a `.htms` file without generating output:

```bash
htms check src/app.htms
```

### Watch Mode

Watch for changes and recompile automatically:

```bash
htms compile src/app.htms -o dist/ --watch
```

### Init

Initialize a new HTMS project (coming soon):

```bash
htms init
```

## Programmatic API

You can also use the CLI programmatically:

```javascript
import { compileFile, checkFile } from '@htms/cli';

const result = await compileFile('src/app.htms', 'dist/');
console.log(result.success); // true/false
```

## Vite Plugin

Use HTMS in your Vite project:

```javascript
// vite.config.js
import { htmsPlugin } from '@htms/cli/vite';

export default {
  plugins: [htmsPlugin({
    include: /\.htms$/,
    outputDir: 'src/generated',
    watch: true
  })]
};
```

### Plugin Options

- `include` - RegExp pattern for files to process (default: `/\.htms$/`)
- `outputDir` - Directory for generated files (default: `'src/generated'`)
- `watch` - Enable watch mode (default: `true`)

## Output Files

The compiler generates three TypeScript files:

1. **templates.ts** - Component/section/page functions
2. **router.ts** - Hash-based router with context management
3. **events.ts** - Event delegation and two-way binding

## Error Formatting

Errors are displayed with source context:

```
error[E002]: Undefined component: 'NavBar'
  --> src/app.htms:10:5

   9 | page home "/" {
  10 |   NavBar
     |   ^^^^^^
  11 | }
```

## License

MIT
