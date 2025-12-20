import chokidar from 'chokidar';
import { compileFile, getStats } from './compiler.js';
import pc from 'picocolors';

/**
 * Watch HTMS files for changes and recompile
 * @param {string} pattern - Glob pattern for files to watch
 * @param {string} outputDir - Output directory
 * @param {object} options - Watch options
 */
export function watchFiles(pattern, outputDir, options = {}) {
  console.log(pc.blue('👁  Watching for changes...'));
  console.log(pc.gray(`   Pattern: ${pattern}`));
  console.log(pc.gray(`   Output:  ${outputDir}\n`));

  const watcher = chokidar.watch(pattern, {
    persistent: true,
    ignoreInitial: false,
    awaitWriteFinish: {
      stabilityThreshold: 100,
      pollInterval: 50
    }
  });

  watcher
    .on('add', path => handleChange(path, outputDir, 'added', options))
    .on('change', path => handleChange(path, outputDir, 'changed', options))
    .on('error', error => console.error(pc.red(`Watcher error: ${error}`)));

  // Handle graceful shutdown
  process.on('SIGINT', () => {
    console.log('\n' + pc.blue('Stopping watcher...'));
    watcher.close();
    process.exit(0);
  });

  return watcher;
}

async function handleChange(filePath, outputDir, action, options) {
  const timestamp = new Date().toLocaleTimeString();
  console.log(pc.gray(`[${timestamp}]`) + ` File ${action}: ${pc.cyan(filePath)}`);

  try {
    const result = await compileFile(filePath, outputDir, { ...options, quiet: false });
    const stats = getStats(result);

    if (result.success) {
      console.log(pc.green('✓') + ` Compiled successfully (${stats.files} files generated)`);
      if (stats.warnings > 0) {
        console.log(pc.yellow('⚠') + ` ${stats.warnings} warning(s)`);
      }
    } else {
      console.log(pc.red('✗') + ` Compilation failed (${stats.errors} error(s))`);
    }
    console.log('');
  } catch (error) {
    console.error(pc.red('Error:') + ` ${error.message}\n`);
  }
}
