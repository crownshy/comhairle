const SWIPE_THRESHOLD_PX = 40;

/** Swipe between slides, ignoring taps, small drags, and vertical scrolls. */
export function carouselSwipe(onPrev: () => void, onNext: () => void) {
	let startX: number | null = null;
	let startY = 0;

	return {
		onpointerdown(event: PointerEvent) {
			startX = event.clientX;
			startY = event.clientY;
		},
		onpointercancel() {
			startX = null;
		},
		onpointerup(event: PointerEvent) {
			if (startX === null) return;
			const dx = event.clientX - startX;
			const dy = event.clientY - startY;
			startX = null;
			// A drag that travelled further down the page than across it was a scroll.
			if (Math.abs(dy) > Math.abs(dx)) return;
			if (dx <= -SWIPE_THRESHOLD_PX) onNext();
			if (dx >= SWIPE_THRESHOLD_PX) onPrev();
		}
	};
}
