import { Node } from '@tiptap/core';

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
				default: ''
			},
			controls: {
				default: true
			},
			autoplay: {
				default: null
			}
		};
	},

	parseHTML() {
		return [
			{
				tag: 'video',
				getAttrs: (node) => ({
					src: node.getAttribute('src'),
					controls: node.getAttribute('controls'),
					autoplay: node.getAttribute('autoplay')
				})
			}
		];
	},

	renderHTML({ HTMLAttributes }) {
		return ['video', HTMLAttributes, 0];
	},

	addCommands() {
		return {
			setVideo:
				(options: VideoOptions) =>
				({ tr, dispatch }) => {
					const { selection } = tr;

					const node = this.type.create({
						...options,
						autoplay: options.autoplay || null
					});

					if (dispatch) {
						tr.replaceRangeWith(selection.from, selection.to, node);
					}

					return true;
				}
		};
	}
});
