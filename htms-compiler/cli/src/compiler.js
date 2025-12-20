import { compile_wasm, init } from '@htms/compiler';
import { readFile, writeFile, mkdir } from 'fs/promises';
import { dirname, join, basename } from 'path';
import { printDiagnostics } from './format-errors.js';
import pc from 'picocolors';

// Initialize WASM module
let wasmInitialized = false;
async function ensureWasmInit() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

/**
 * Compile HTMS file to TypeScript
 * @param {string} inputPath - Path to .htms file
 * @param {string} outputDir - Output directory for generated files
 * @param {object} options - Compilation options
 * @returns {Promise<{success: boolean, diagnostics: Array}>}
 */
export async function compileFile(inputPath, outputDir, options = {}) {
  await ensureWasmInit();

  // Read source file
  const source = await readFile(inputPath, 'utf-8');

  // Compile using WASM
  const result = compile_wasm(source);

  // Print diagnostics
  if (result.diagnostics && result.diagnostics.length > 0) {
    printDiagnostics(result.diagnostics, source, inputPath);
  }

  // Write output files if successful
  if (result.success && result.files) {
    await mkdir(outputDir, { recursive: true });

    for (const file of result.files) {
      const outputPath = join(outputDir, file.path);
      await writeFile(outputPath, file.content, 'utf-8');

      if (!options.quiet) {
        console.log(pc.green('✓') + ` Generated ${pc.cyan(file.path)}`);
      }
    }
  }

  return result;
}

/**
 * Check HTMS file for errors without generating output
 * @param {string} inputPath - Path to .htms file
 * @returns {Promise<{success: boolean, diagnostics: Array}>}
 */
export async function checkFile(inputPath) {
  await ensureWasmInit();

  // Read source file
  const source = await readFile(inputPath, 'utf-8');

  // Compile (but don't write output)
  const result = compile_wasm(source);

  // Print diagnostics
  if (result.diagnostics && result.diagnostics.length > 0) {
    printDiagnostics(result.diagnostics, source, inputPath);
  }

  return result;
}

/**
 * Get compilation statistics
 * @param {object} result - Compilation result
 * @returns {object} Statistics
 */
export function getStats(result) {
  const errors = result.diagnostics?.filter(d => d.severity === 'Error').length || 0;
  const warnings = result.diagnostics?.filter(d => d.severity === 'Warning').length || 0;
  const files = result.files?.length || 0;

  return { errors, warnings, files };
}
