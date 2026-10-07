import type PhotoSwipe from 'photoswipe';
import type { ZoomLevelOption } from 'photoswipe';
import * as m from '$lib/paraglide/messages';
import { tryCatchAsync } from '$lib/utils/errorHandling';

const OPEN_HEIGHT_RATIO = 0.8;
const CLOSE_SWIPE_RATIO = 0.15;

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

// PhotoSwipe only closes on a vertical swipe at fit zoom, and wide images open above fit.
// Close on a long vertical swipe whenever the image has no vertical room to pan.
function closeOnVerticalSwipe(lightbox: PhotoSwipe) {
	lightbox.on('pointerUp', () => {
		const { gestures, currSlide, viewportSize } = lightbox;
		if (!currSlide || currSlide.currZoomLevel <= currSlide.zoomLevels.fit) return;
		if (!gestures.isDragging || gestures.isMultitouch || gestures.dragAxis !== 'y') return;

		const canPanVertically = currSlide.bounds.min.y !== currSlide.bounds.max.y;
		const swipeDistance = Math.abs(gestures.p1.y - gestures.startP1.y);
		if (!canPanVertically && swipeDistance > viewportSize.y * CLOSE_SWIPE_RATIO) {
			lightbox.close();
		}
	});
}

/**
 * Opens the image full screen in a zoomable PhotoSwipe lightbox. PhotoSwipe loads on
 * first open so pages without a clicked image don't pay for it. If it fails to load, the
 * image stays inline as it was.
 */
export async function openImageLightbox(image: HTMLImageElement) {
	const src = image.currentSrc || image.src;
	if (!src || !image.naturalWidth) return;

	const loaded = await tryCatchAsync(() =>
		Promise.all([import('photoswipe'), import('photoswipe/style.css')])
	);
	if (loaded.err !== null) return;
	const PhotoSwipeLightbox = loaded.ok[0].default;

	const lightbox = new PhotoSwipeLightbox({
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
	closeOnVerticalSwipe(lightbox);
	lightbox.init();
}
