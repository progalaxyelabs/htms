#!/usr/bin/env node

import { Command } from 'commander';
import { compileFile, checkFile, getStats } from '../src/compiler.js';
import { watchFiles } from '../src/watch.js';
import { readFile } from 'fs/promises';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import pc from 'picocolors';

const __dirname = dirname(fileURLToPath(import.meta.url));
const packageJson = JSON.parse(
  await readFile(join(__dirname, '../package.json'), 'utf-8')
);

const program = new Command();

program
  .name('htms')
  .description('HTMS compiler - compile .htms files to TypeScript')
  .version(packageJson.version);

// Compile command
program
  .command('compile')
  .description('Compile .htms file to TypeScript or HTML')
  .argument('<input>', 'Input .htms file')
  .option('-o, --output <dir>', 'Output directory', 'dist')
  .option('-f, --format <format>', 'Output format: typescript or html', 'typescript')
  .option('-t, --template <file>', 'HTML template file to inject into (only for html format)')
  .option('-s, --split-templates', 'Split templates into separate files for lazy loading (only for html format)')
  .option('-w, --watch', 'Watch for changes')
  .option('-q, --quiet', 'Suppress output')
  .action(async (input, options) => {
    try {
      if (options.watch) {
        watchFiles(input, options.output, options);
      } else {
        const result = await compileFile(input, options.output, options);
        const stats = getStats(result);

        if (result.success) {
          if (!options.quiet) {
            console.log(pc.green('\n✓ Compilation successful!'));
            console.log(pc.gray(`  Generated ${stats.files} file(s)`));
            if (stats.warnings > 0) {
              console.log(pc.yellow(`  ${stats.warnings} warning(s)`));
            }
          }
          process.exit(0);
        } else {
          if (!options.quiet) {
            console.error(pc.red('\n✗ Compilation failed!'));
            console.error(pc.gray(`  ${stats.errors} error(s)`));
          }
          process.exit(1);
        }
      }
    } catch (error) {
      console.error(pc.red('Error:'), error.message);
      console.error(error.stack);
      process.exit(1);
    }
  });

// Check command
program
  .command('check')
  .description('Check .htms file for errors without generating output')
  .argument('<input>', 'Input .htms file')
  .action(async (input) => {
    try {
      const result = await checkFile(input);
      const stats = getStats(result);

      if (result.success) {
        console.log(pc.green('\n✓ No errors found!'));
        if (stats.warnings > 0) {
          console.log(pc.yellow(`  ${stats.warnings} warning(s)`));
        }
        process.exit(0);
      } else {
        console.error(pc.red('\n✗ Validation failed!'));
        console.error(pc.gray(`  ${stats.errors} error(s)`));
        process.exit(1);
      }
    } catch (error) {
      console.error(pc.red('Error:'), error.message);
      process.exit(1);
    }
  });

// Init command (scaffold new project)
program
  .command('init')
  .description('Initialize a new HTMS project')
  .option('-d, --dir <directory>', 'Project directory', '.')
  .action(async (options) => {
    console.log(pc.blue('📦 Initializing new HTMS project...'));
    console.log(pc.gray(`   Directory: ${options.dir}\n`));

    // This would create a basic project structure
    // For now, just show what would be created
    console.log(pc.gray('Would create:'));
    console.log(pc.gray('  src/'));
    console.log(pc.gray('    app.htms'));
    console.log(pc.gray('    actions.ts'));
    console.log(pc.gray('    runtime.ts'));
    console.log(pc.gray('  dist/'));
    console.log(pc.gray('  package.json'));
    console.log(pc.gray('  vite.config.ts\n'));

    console.log(pc.yellow('⚠  Init command not yet implemented'));
    console.log(pc.gray('   This is a placeholder for future functionality'));
  });

program.parse();
