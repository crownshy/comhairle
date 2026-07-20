import type { MediaContentType } from '@crownshy/api-client/api';

export type HTMLMediaElement = 'audio' | 'image' | 'video';
/**
 * Function to convert from Media type to a html type
 * htmlTypeFromMediaType("audio/mpeg") => "audio"
 * htmlTypeFromMediaType("image/jpeg") => "image"
 * htmlTypeFromMediaType("video/mp4") => "video"
 */
export function htmlFromMediaType(content: MediaContentType): HTMLMediaElement | undefined {
	// TODO: Change type back to MediaContentType
	const type = content.split('/')[0];
	if (type === 'audio' || type === 'image' || type === 'video') {
		return type;
	}
	return undefined;
}
