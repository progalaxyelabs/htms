/**
 * HTMS CLI - Main exports
 */

export { compileFile, checkFile, getStats } from './compiler.js';
export { watchFiles } from './watch.js';
export { formatDiagnostics, printDiagnostics } from './format-errors.js';
export { htmsPlugin } from './vite-plugin.js';
