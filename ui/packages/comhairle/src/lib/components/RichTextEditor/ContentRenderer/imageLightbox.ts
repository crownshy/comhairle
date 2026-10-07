import type { ZoomLevelOption } from 'photoswipe';
import * as m from '$lib/paraglide/messages';

const OPEN_HEIGHT_RATIO = 0.8;

type ZoomLevel = Parameters<Extract<ZoomLevelOption, (...args: never[]) => number>>[0];

// Wide images fitted to a phone's width come out too short to read, so open tall enough
// to fill most of the screen and let people pan sideways.
function openingZoom({ fit, panAreaSize, elementSize }: ZoomLevel): number {
	if (!panAreaSize || !elementSize) return fit;
	return Math.max(fit, (panAreaSize.y * OPEN_HEIGHT_RATIO) / elementSize.y);
}

// PhotoSwipe's default zoom target can sit below the opening zoom, which would make the
// zoom button shrink the image.
function zoomButtonTarget(zoomLevel: ZoomLevel): number {
	return Math.max(1, openingZoom(zoomLevel) * 2);
}

// Loaded on first open so pages without a clicked image don't pay for PhotoSwipe.
export async function openImageLightbox(image: HTMLImageElement) {
	const src = image.currentSrc || image.src;
	if (!src || !image.naturalWidth) return;

	const [{ default: PhotoSwipe }] = await Promise.all([
		import('photoswipe'),
		import('photoswipe/style.css')
	]);

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
		initialZoomLevel: openingZoom,
		secondaryZoomLevel: zoomButtonTarget,
		wheelToZoom: true,
		showHideAnimationType: 'zoom',
		closeTitle: m.image_lightbox_close(),
		zoomTitle: m.image_lightbox_zoom()
	});
	lightbox.init();
}
