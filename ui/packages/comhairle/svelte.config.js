import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const TRUSTED_SOURCES = [
	'https://jitsi.comhairle.scot',
	'https://forms.comhairle.scot',
	'https://polis.comhairle.scot'
];

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://svelte.dev/docs/kit/adapters for more information about adapters.
		adapter: adapter(),
		csp: {
			mode: 'auto', // uses hashes for prerendered, nonces for SSR
			directives: {
				'default-src': ['self'],
				'script-src': ['self'],
				'style-src': ['self', 'unsafe-inline'], // needed if you use Svelte transitions
				'img-src': ['self', 'data:'],
				'frame-src': ['self', ...TRUSTED_SOURCES],
				'connect-src': ['self', 'wss://jitsi.comhairle.scot'],
				// 'frame-ancestors': ['none'], // iframe ancestors policy via CSP
				'object-src': ['none'],
				'base-uri': ['self']
			}
		}
	},
	extensions: ['.svelte', '.svx']
};

export default config;
