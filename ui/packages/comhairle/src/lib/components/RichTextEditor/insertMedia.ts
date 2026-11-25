import type { Editor } from '@tiptap/core';
import { isVideoFileUrl } from '$lib/utils/videoUrl';

export type VideoSubmission = {
	from: 'library' | 'external-url';
	url: string;
	title?: string | null;
	captions?: string | null;
};

/** Inserts a video file as a playable video and any other video URL as an embed. */
export function insertVideo(editor: Editor | undefined, video: VideoSubmission) {
	const chain = editor?.chain().focus();
	if (video.from === 'library' || isVideoFileUrl(video.url)) {
		chain?.setVideo({ src: video.url, captions: video.captions }).run();
	} else {
		chain?.setIframe({ src: video.url, title: video.title }).run();
	}
}
