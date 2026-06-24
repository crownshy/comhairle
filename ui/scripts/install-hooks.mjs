import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';

let gitRoot;
try {
	gitRoot = execSync('git rev-parse --show-toplevel', { encoding: 'utf-8' }).trim();
} catch {
	console.log('[install-hooks] Not a git repository, skipping hook installation');
	process.exit(0);
}

const installerScript = resolve(gitRoot, '.githooks', 'install-hooks.sh');

if (!existsSync(installerScript)) {
	console.error(`[install-hooks] Error: Installer script not found at ${installerScript}`);
	process.exit(1);
}

try {
	// Execute the shell script using bash
	// We pass the gitRoot so the shell script knows the context
	execSync(`bash "${installerScript}"`, {
		stdio: 'inherit',
		env: { ...process.env, REPO_BASE: gitRoot }
	});
	console.log('[install-hooks] Pre-commit hooks successfully installed via shell script.');
} catch (error) {
	console.error('[install-hooks] Failed to execute installation script:', error.message);
	process.exit(1);
}
