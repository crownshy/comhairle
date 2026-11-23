const VIDEO_FILE_EXTENSIONS = ['.mp4', '.webm', '.mov', '.m4v', '.ogv'];

/**
 * True when the URL points straight at a video file rather than a player page like YouTube.
 * @param url - Any URL string; invalid URLs return false.
 */
export function isVideoFileUrl(url: string | null | undefined): boolean {
	if (!url) return false;

	try {
		const pathname = new URL(url).pathname.toLowerCase();
		return VIDEO_FILE_EXTENSIONS.some((extension) => pathname.endsWith(extension));
	} catch {
		return false;
	}
}

/**
 * Adds a `#t=0.001` media fragment so iOS loads the first frame and shows it as a thumbnail.
 * iOS skips loading video data until play otherwise, which leaves a blank box.
 * @param url - The video file URL, without a fragment of its own.
 */
export function withFirstFrameThumbnail(url: string): string {
	if (!url || url.includes('#')) return url;
	return `${url}#t=0.001`;
}

/**
 * Removes the fragment added by {@link withFirstFrameThumbnail} so it isn't stored twice.
 * @param url - A video src read back from rendered HTML.
 */
export function withoutFirstFrameThumbnail(url: string): string {
	return url.replace(/#t=0\.001$/, '');
}
