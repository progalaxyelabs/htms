import { compile_wasm, init } from '@htms/compiler';
import { readFile, writeFile } from 'fs/promises';
import { dirname, join, relative } from 'path';
import pc from 'picocolors';

let wasmInitialized = false;

/**
 * Vite plugin for HTMS compilation
 * @param {object} options - Plugin options
 * @returns {import('vite').Plugin}
 */
export function htmsPlugin(options = {}) {
  const {
    include = /\.htms$/,
    outputDir = 'src/generated',
    watch = true
  } = options;

  return {
    name: 'vite-plugin-htms',

    async buildStart() {
      if (!wasmInitialized) {
        await init();
        wasmInitialized = true;
      }
    },

    async handleHotUpdate({ file, server }) {
      // Only process .htms files
      if (!include.test(file)) {
        return;
      }

      console.log(pc.blue('[htms]') + ` Compiling ${pc.cyan(relative(process.cwd(), file))}`);

      try {
        const source = await readFile(file, 'utf-8');
        const result = compile_wasm(source);

        if (result.success && result.files) {
          // Write generated files
          for (const genFile of result.files) {
            const outputPath = join(outputDir, genFile.path);
            await writeFile(outputPath, genFile.content, 'utf-8');
            console.log(pc.green('  ✓') + ` Generated ${pc.gray(genFile.path)}`);
          }

          // Trigger HMR for generated files
          const modules = [];
          for (const genFile of result.files) {
            const modulePath = join(outputDir, genFile.path);
            const module = server.moduleGraph.getModuleById(modulePath);
            if (module) {
              modules.push(module);
            }
          }

          if (modules.length > 0) {
            return modules;
          }
        } else {
          // Print errors
          console.error(pc.red('  ✗') + ` Compilation failed`);
          for (const diag of result.diagnostics || []) {
            if (diag.severity === 'Error') {
              console.error(pc.red(`    ${diag.message}`) + pc.gray(` at line ${diag.location.line}`));
            }
          }
        }
      } catch (error) {
        console.error(pc.red('[htms]') + ` Error: ${error.message}`);
      }

      return [];
    },

    async transform(code, id) {
      // Only process .htms files
      if (!include.test(id)) {
        return null;
      }

      try {
        const result = compile_wasm(code);

        if (!result.success) {
          // Show errors in console
          console.error(pc.red('[htms]') + ` Compilation failed for ${id}`);
          for (const diag of result.diagnostics || []) {
            if (diag.severity === 'Error') {
              console.error(pc.red(`  ${diag.message}`) + pc.gray(` at line ${diag.location.line}`));
            }
          }

          // Return error as module
          const errorMsg = result.diagnostics
            ?.filter(d => d.severity === 'Error')
            .map(d => `${d.message} (line ${d.location.line})`)
            .join('\\n');

          return {
            code: `throw new Error('HTMS compilation failed:\\n${errorMsg}');`,
            map: null
          };
        }

        // Write generated files to output directory
        if (result.files) {
          for (const file of result.files) {
            const outputPath = join(outputDir, file.path);
            await writeFile(outputPath, file.content, 'utf-8');
          }
        }

        // Return a module that imports the generated templates
        return {
          code: `export { default } from '${outputDir}/templates.js';`,
          map: null
        };
      } catch (error) {
        console.error(pc.red('[htms]') + ` Error processing ${id}:`, error.message);
        return {
          code: `throw new Error('HTMS plugin error: ${error.message}');`,
          map: null
        };
      }
    }
  };
}

export default htmsPlugin;
