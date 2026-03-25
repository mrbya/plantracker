import { cpSync, existsSync, mkdirSync, rmSync, copyFile } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(__dirname, '..');
const docsSiteDir = join(repoRoot, 'docs-page');
const cargoDocDir = join(repoRoot, 'src-tauri', 'target', 'doc');
const landigPage = join(repoRoot, 'docs', 'landing-page', 'index.html')

function run(command) {
  console.log(`\n> ${command}`);
  execSync(command, {
    cwd: repoRoot,
    stdio: 'inherit',
    env: process.env
  });
}

function cleanDir(path) {
  rmSync(path, { recursive: true, force: true });
  mkdirSync(path, { recursive: true });
}

function copyDir(from, to) {
  rmSync(to, { recursive: true, force: true });
  mkdirSync(dirname(to), { recursive: true });
  cpSync(from, to, { recursive: true });
}

function writeLandingPage() {
  copyFile(landigPage, join(docsSiteDir, 'index.html'), (err) => {
    if (err) throw err;
    console.log('Copied landing page.');
  })
}

cleanDir(docsSiteDir);

run('just docs-rs');
run('just docs-api');
run('just docs-ui');

if (!existsSync(cargoDocDir)) {
  throw new Error(`cargo doc output not found at: ${cargoDocDir}`);
}

copyDir(cargoDocDir, join(docsSiteDir, 'backend'));
writeLandingPage();

console.log(`\nDocumentation site generated at: ${docsSiteDir}`);
