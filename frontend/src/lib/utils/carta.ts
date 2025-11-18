import { Carta, type Plugin } from 'carta-md';
import { slash } from '@cartamd/plugin-slash';
import { video } from 'carta-plugin-video';
import DOMPurify from 'isomorphic-dompurify';
import rehypeRaw from 'rehype-raw';

const sanitizeOptions = {
	ADD_ATTR: ['target']
};

const htmlPlugin: Plugin = {
	transformers: [
		{
			execution: 'sync',
			type: 'rehype',
			transform({ processor }) {
				processor.use(rehypeRaw);
			}
		}
	]
};

/**
 * Creates a standard Carta instance with consistent configuration across the app
 * @param includeSlash Whether to include the slash plugin (default: true)
 * @returns Configured Carta instance
 */
export function createCarta(includeSlash: boolean = true): Carta {
	const extensions = [video(), htmlPlugin];
	if (includeSlash) {
		extensions.unshift(slash());
	}

	return new Carta({
		sanitizer: (html) => DOMPurify.sanitize(html, sanitizeOptions),
		extensions,
		rehypeOptions: {
			allowDangerousHtml: true
		}
	});
}
