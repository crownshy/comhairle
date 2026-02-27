import { execSync } from 'node:child_process';
import { writeFileSync, chmodSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';

let gitRoot;
try {
	gitRoot = execSync('git rev-parse --show-toplevel', { encoding: 'utf-8' }).trim();
} catch {
	console.log('[install-hooks] Not a git repository, skipping hook installation');
	process.exit(0);
}
const hooksDir = resolve(gitRoot, '.git', 'hooks');
const hookPath = resolve(hooksDir, 'pre-commit');

const hookScript = `#!/bin/sh
./ui/node_modules/.bin/pretty-quick --staged
`;

mkdirSync(hooksDir, { recursive: true });
writeFileSync(hookPath, hookScript);
chmodSync(hookPath, '755');

console.log('[install-hooks] pre-commit hook installed');
