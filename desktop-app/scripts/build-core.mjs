import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

console.log('Building core...');
execSync('cargo build --release', {
  cwd: path.join(__dirname, '../../rust-core'),
  stdio: 'inherit',
});

const source = path.join(__dirname, '../../rust-core/target/release/rust-core');
const dest = path.join(__dirname, '../resources/rust-core');

if (!fs.existsSync(path.dirname(dest))) {
  fs.mkdirSync(path.dirname(dest), { recursive: true });
}

fs.copyFileSync(source, dest);
console.log('Rust binary copied to resources/');
