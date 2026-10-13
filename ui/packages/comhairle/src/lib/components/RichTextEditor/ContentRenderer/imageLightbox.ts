import type { ZoomLevelOption } from 'photoswipe';
import { m } from '$lib/paraglide/messages';
import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';

type ZoomLevel = Parameters<Extract<ZoomLevelOption, (...args: never[]) => number>>[0];

// The lightbox opens at fit so people see the whole image. Double-tap or the zoom button
// then makes it readable: at least twice the size, and tall enough to fill the screen so
// wide diagrams on a phone can be panned sideways.
function readableZoom({ fit, panAreaSize, elementSize }: ZoomLevel): number {
	if (!panAreaSize || !elementSize) return fit * 2;
	return Math.max(fit * 2, panAreaSize.y / elementSize.y);
}

/**
 * Opens the image full screen in a zoomable PhotoSwipe lightbox. PhotoSwipe loads on
 * first open so pages without a clicked image don't pay for it. If it fails to load, the
 * image stays inline and a toast says so, since the zoom cursor promised something would
 * happen.
 */
export async function openImageLightbox(image: HTMLImageElement) {
	const src = image.currentSrc || image.src;
	if (!src || !image.naturalWidth) return;

	const loaded = await tryCatchAsync(() =>
		Promise.all([import('photoswipe'), import('photoswipe/style.css')])
	);
	if (loaded.err !== null) {
		notifications.send({
			priority: 'ERROR',
			message: m.image_lightbox_load_failed(),
			duration: 5000
		});
		return;
	}
	const PhotoSwipe = loaded.ok[0].default;

	const lightbox = new PhotoSwipe({
		dataSource: [
			{
				src,
				alt: image.alt,
				width: image.naturalWidth,
				height: image.naturalHeight,
				element: image
			}
		],
		secondaryZoomLevel: readableZoom,
		bgOpacity: 1,
		wheelToZoom: true,
		showHideAnimationType: 'zoom',
		closeTitle: m.image_lightbox_close(),
		zoomTitle: m.image_lightbox_zoom()
	});
	lightbox.init();
}
