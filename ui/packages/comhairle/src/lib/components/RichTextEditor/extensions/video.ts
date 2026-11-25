import { Node } from '@tiptap/core';
import {
	isVideoFileUrl,
	withFirstFrameThumbnail,
	withoutFirstFrameThumbnail
} from '$lib/utils/videoUrl';
import { validateUrl } from '$lib/utils/urlValidation';
import { m } from '$lib/paraglide/messages';

export interface VideoOptions {
	src: string;
	controls?: boolean;
	autoplay?: boolean;
	/** URL of a WebVTT captions file, shown as a captions track participants can turn on. */
	captions?: string | null;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		video: {
			setVideo: (options: VideoOptions) => ReturnType;
		};
	}
}

function readCaptionsSrc(element: HTMLElement): string | null {
	return element.querySelector('track[kind="captions"]')?.getAttribute('src') ?? null;
}

function readVideoSrc(element: HTMLElement): string {
	const src = element.getAttribute('src') ?? element.querySelector('source')?.getAttribute('src');
	return withoutFirstFrameThumbnail(src ?? '');
}

export const Video = Node.create<VideoOptions>({
	name: 'video',
	group: 'block',
	atom: true,

	addOptions() {
		return {
			src: '',
			controls: true,
			autoplay: false
		};
	},

	addAttributes() {
		return {
			src: {
				default: '',
				parseHTML: readVideoSrc
			},
			controls: {
				default: true,
				parseHTML: (element) => element.hasAttribute('controls')
			},
			autoplay: {
				default: false,
				parseHTML: (element) => element.hasAttribute('autoplay')
			},
			captions: {
				default: null,
				parseHTML: readCaptionsSrc
			}
		};
	},

	parseHTML() {
		return [
			{ tag: 'video' },
			// Videos added before this node existed were stored as iframes pointing at the file,
			// which browsers autoplay. Read them back as video nodes so old content is fixed too.
			{
				tag: 'iframe',
				priority: 60,
				getAttrs: (element) => (isVideoFileUrl(element.getAttribute('src')) ? {} : false)
			}
		];
	},

	renderHTML({ node }) {
		const { src, controls, autoplay } = node.attrs;
		const captions = validateUrl(node.attrs.captions);
		const source = ['source', { src: withFirstFrameThumbnail(src) }];

		// playsinline stops iPhones forcing fullscreen playback. Browsers only load a captions
		// file from another origin when the video is requested with CORS, hence crossorigin.
		return [
			'video',
			{
				controls: controls ? '' : null,
				autoplay: autoplay ? '' : null,
				playsinline: '',
				preload: 'metadata',
				crossorigin: captions ? 'anonymous' : null,
				class: 'w-full rounded-lg'
			},
			source,
			...(captions
				? [['track', { kind: 'captions', src: captions, label: m.rich_text_captions() }]]
				: [])
		];
	},

	addCommands() {
		return {
			setVideo:
				(options: VideoOptions) =>
				({ tr, dispatch }) => {
					const { selection } = tr;
					const node = this.type.create(options);

					if (dispatch) {
						tr.replaceRangeWith(selection.from, selection.to, node);
					}

					return true;
				}
		};
	}
});
