import pc from 'picocolors';

/**
 * Format diagnostics with source context
 * @param {Array} diagnostics - Array of diagnostic objects
 * @param {string} source - Original source code
 * @param {string} filename - Filename for display
 * @returns {string} Formatted error messages
 */
export function formatDiagnostics(diagnostics, source, filename = 'input.htms') {
  if (diagnostics.length === 0) {
    return '';
  }

  const lines = source.split('\n');
  const output = [];

  for (const diag of diagnostics) {
    const { severity, message, location, code } = diag;
    const { line, column } = location;

    // Header with severity
    const severityStr = severity === 'Error'
      ? pc.red(pc.bold('error'))
      : pc.yellow(pc.bold('warning'));

    const codeStr = code ? pc.gray(`[${code}]`) : '';
    output.push(`${severityStr}${codeStr}: ${message}`);

    // Location
    output.push(pc.cyan(`  --> ${filename}:${line}:${column}`));
    output.push('');

    // Source context (line before, error line, line after)
    const startLine = Math.max(1, line - 1);
    const endLine = Math.min(lines.length, line + 1);

    for (let i = startLine; i <= endLine; i++) {
      const lineNum = String(i).padStart(4, ' ');
      const sourceLine = lines[i - 1] || '';

      if (i === line) {
        // Error line
        output.push(pc.gray(`${lineNum} | `) + sourceLine);

        // Underline the error
        const padding = ' '.repeat(column - 1);
        const underline = pc.red('^'.repeat(Math.max(1, sourceLine.length - column + 1)));
        output.push(pc.gray('     | ') + padding + underline);
      } else {
        // Context line
        output.push(pc.gray(`${lineNum} | ${sourceLine}`));
      }
    }

    output.push('');
  }

  return output.join('\n');
}

/**
 * Print diagnostics to console
 * @param {Array} diagnostics - Array of diagnostic objects
 * @param {string} source - Original source code
 * @param {string} filename - Filename for display
 */
export function printDiagnostics(diagnostics, source, filename) {
  const formatted = formatDiagnostics(diagnostics, source, filename);
  if (formatted) {
    console.error(formatted);
  }
}
