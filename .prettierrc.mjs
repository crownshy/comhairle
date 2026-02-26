import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(
	join(__dirname, 'ui', 'packages', 'comhairle', 'node_modules', 'prettier', 'index.js')
);

export default {
	useTabs: true,
	tabWidth: 4,
	singleQuote: true,
	trailingComma: 'none',
	printWidth: 100,
	plugins: [
		require.resolve('prettier-plugin-svelte'),
		require.resolve('prettier-plugin-tailwindcss')
	],
	overrides: [
		{
			files: '*.svelte',
			options: {
				parser: 'svelte'
			}
		}
	]
};
