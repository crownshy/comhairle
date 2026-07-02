import type { ContentType } from '../../routes/(admin)/admin/media-library/+page.server';

type HTMLMediaElement = 'audio' | 'img' | 'video';
/**
 * Function to convert from Media type to a html type
 * htmlTypeFromMediaType("audio/mp3") => "audio"
 * htmlTypeFromMediaType("image/jpeg") => "image"
 * htmlTypeFromMediaType("video/mp4") => "video"
 */
export function htmlFromMediaType(content: ContentType): HTMLMediaElement | undefined {
	// TODO: Change type back to MediaContentType
	const type = content.split('/')[0];
	if (type === 'audio' || type === 'image' || type === 'video') {
		if (type === 'image') {
			return 'img';
		}
		return type;
	}
	return undefined;
}
