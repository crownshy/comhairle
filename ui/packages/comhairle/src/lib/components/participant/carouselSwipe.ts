const SWIPE_THRESHOLD_PX = 40;

/** Swipe between slides, ignoring taps and small drags. */
export function carouselSwipe(onPrev: () => void, onNext: () => void) {
	let startX = 0;

	return {
		onpointerdown(event: PointerEvent) {
			startX = event.clientX;
		},
		onpointerup(event: PointerEvent) {
			const dx = event.clientX - startX;
			if (dx <= -SWIPE_THRESHOLD_PX) onNext();
			if (dx >= SWIPE_THRESHOLD_PX) onPrev();
		}
	};
}
