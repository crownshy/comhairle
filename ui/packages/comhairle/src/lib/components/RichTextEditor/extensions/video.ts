import { Node } from '@tiptap/core';
import {
	isVideoFileUrl,
	withFirstFrameThumbnail,
	withoutFirstFrameThumbnail
} from '$lib/utils/videoUrl';

export interface VideoOptions {
	src: string;
	controls?: boolean;
	autoplay?: boolean;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		video: {
			setVideo: (options: VideoOptions) => ReturnType;
		};
	}
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

		// playsinline stops iPhones forcing fullscreen playback.
		return [
			'video',
			{
				controls: controls ? '' : null,
				autoplay: autoplay ? '' : null,
				playsinline: '',
				preload: 'metadata',
				class: 'w-full rounded-lg'
			},
			['source', { src: withFirstFrameThumbnail(src) }]
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
