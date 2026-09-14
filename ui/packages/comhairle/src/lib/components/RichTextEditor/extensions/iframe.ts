import { Node } from '@tiptap/core';
import {
	validateIframeUrl,
	isVideoFileUrl,
	DEFAULT_ALLOWED_DOMAINS
} from '$lib/utils/urlValidation';

export interface IframeOptions {
	allowFullscreen: boolean;
	HTMLAttributes: Record<string, string | number | boolean>;
	allowedDomains?: string[];
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		iframe: {
			setIframe: (options: { src: string }) => ReturnType;
		};
	}
}

export const Iframe = Node.create<IframeOptions>({
	name: 'iframe',

	group: 'block',

	atom: true,

	addOptions() {
		return {
			allowFullscreen: true,
			HTMLAttributes: {
				class: 'iframe-wrapper'
			},
			allowedDomains: DEFAULT_ALLOWED_DOMAINS
		};
	},

	addAttributes() {
		return {
			src: {
				default: null,
				parseHTML: (element) => {
					const src = element.getAttribute('src');
					return validateIframeUrl(
						src,
						this.options.allowedDomains || DEFAULT_ALLOWED_DOMAINS
					);
				},
				renderHTML: (attributes) => {
					const validSrc = validateIframeUrl(
						attributes.src,
						this.options.allowedDomains || DEFAULT_ALLOWED_DOMAINS
					);

					if (!validSrc) {
						return {};
					}

					return { src: validSrc };
				}
			},
			frameborder: {
				default: 0,
				parseHTML: (element) => {
					const value = element.getAttribute('frameborder');
					return value ? parseInt(value, 10) : 0;
				}
			},
			allowfullscreen: {
				default: this.options.allowFullscreen,
				parseHTML: (element) => {
					return element.hasAttribute('allowfullscreen') || this.options.allowFullscreen;
				},
				renderHTML: (attributes) => {
					if (attributes.allowfullscreen) {
						return { allowfullscreen: '' };
					}
					return {};
				}
			},
			sandbox: {
				default: 'allow-scripts allow-same-origin allow-presentation allow-popups',
				parseHTML: (element) => {
					return (
						element.getAttribute('sandbox') ||
						'allow-scripts allow-same-origin allow-presentation allow-popups'
					);
				}
			},
			referrerpolicy: {
				default: 'no-referrer-when-downgrade',
				parseHTML: (element) => {
					return element.getAttribute('referrerpolicy') || 'no-referrer-when-downgrade';
				}
			}
		};
	},

	parseHTML() {
		return [
			{
				tag: 'iframe',
				getAttrs: (element) => {
					if (typeof element === 'string') return false;

					const src = element.getAttribute('src');
					const validSrc = validateIframeUrl(
						src,
						this.options.allowedDomains || DEFAULT_ALLOWED_DOMAINS
					);

					if (!validSrc) {
						return false;
					}

					return {};
				}
			},
			// What renderHTML below emits for a video file, so copying one and pasting it back
			// keeps the node.
			{
				tag: 'video[src]',
				getAttrs: (element) => {
					if (typeof element === 'string') return false;
					const src = element.getAttribute('src');
					return validateIframeUrl(
						src,
						this.options.allowedDomains || DEFAULT_ALLOWED_DOMAINS
					)
						? {}
						: false;
				}
			}
		];
	},

	renderHTML({ HTMLAttributes }) {
		const validSrc = validateIframeUrl(
			HTMLAttributes.src,
			this.options.allowedDomains || DEFAULT_ALLOWED_DOMAINS
		);

		if (!validSrc) {
			return [
				'div',
				{
					...this.options.HTMLAttributes,
					class: `${this.options.HTMLAttributes.class} iframe-blocked`
				},
				[
					'p',
					{ style: 'text-align: center; padding: 2rem; color: #666;' },
					'⚠️ Video embed blocked for security reasons'
				]
			];
		}

		// A video file in an iframe opens the browser's own media page: the player drawn at
		// the file's intrinsic size in a black frame until its metadata arrives, then resized,
		// with a buffering spinner over it. A <video> fills the wrapper from its first paint.
		// `#t=0.001` is what makes iOS Safari paint the first frame instead of an empty box.
		if (isVideoFileUrl(validSrc)) {
			return [
				'div',
				this.options.HTMLAttributes,
				[
					'video',
					{
						src: validSrc.includes('#') ? validSrc : `${validSrc}#t=0.001`,
						controls: '',
						playsinline: '',
						preload: 'metadata'
					}
				]
			];
		}

		return [
			'div',
			this.options.HTMLAttributes,
			[
				'iframe',
				{
					...HTMLAttributes,
					src: validSrc,
					sandbox:
						HTMLAttributes.sandbox ||
						'allow-scripts allow-same-origin allow-presentation allow-popups',
					referrerpolicy: HTMLAttributes.referrerpolicy || 'no-referrer-when-downgrade'
				}
			]
		];
	},

	addCommands() {
		return {
			setIframe:
				(options: { src: string }) =>
				({ tr, dispatch, state }) => {
					const validSrc = validateIframeUrl(
						options.src,
						this.options.allowedDomains || DEFAULT_ALLOWED_DOMAINS
					);

					const { selection } = tr;

					const node = this.type.create({
						src: validSrc || options.src,
						allowfullscreen: this.options.allowFullscreen,
						sandbox: 'allow-scripts allow-same-origin allow-presentation allow-popups',
						referrerpolicy: 'no-referrer-when-downgrade'
					});

					if (!validSrc) {
						console.warn('[Iframe] Blocked invalid iframe URL:', options.src);
					}

					if (dispatch) {
						tr.replaceRangeWith(selection.from, selection.to, node);
					}

					return true;
				}
		};
	}
});
