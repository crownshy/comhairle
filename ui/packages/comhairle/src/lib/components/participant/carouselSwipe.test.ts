import { describe, expect, it, vi } from 'vitest';
import { carouselSwipe } from './carouselSwipe';

function pointer(x: number, y: number) {
	return { clientX: x, clientY: y } as PointerEvent;
}

function drag(swipe: ReturnType<typeof carouselSwipe>, dx: number, dy: number) {
	swipe.onpointerdown(pointer(100, 100));
	swipe.onpointerup(pointer(100 + dx, 100 + dy));
}

describe('carouselSwipe', () => {
	it('advances on a leftward drag and goes back on a rightward one', () => {
		const onPrev = vi.fn();
		const onNext = vi.fn();
		const swipe = carouselSwipe(onPrev, onNext);

		drag(swipe, -60, 0);
		expect(onNext).toHaveBeenCalledOnce();

		drag(swipe, 60, 0);
		expect(onPrev).toHaveBeenCalledOnce();
	});

	it('ignores taps and short drags', () => {
		const onPrev = vi.fn();
		const onNext = vi.fn();
		const swipe = carouselSwipe(onPrev, onNext);

		drag(swipe, 0, 0);
		drag(swipe, -20, 0);

		expect(onPrev).not.toHaveBeenCalled();
		expect(onNext).not.toHaveBeenCalled();
	});

	it('ignores a drag that travelled further vertically', () => {
		const onNext = vi.fn();
		const swipe = carouselSwipe(vi.fn(), onNext);

		drag(swipe, -60, -200);

		expect(onNext).not.toHaveBeenCalled();
	});

	it('ignores a pointerup after the gesture was cancelled', () => {
		const onNext = vi.fn();
		const swipe = carouselSwipe(vi.fn(), onNext);

		swipe.onpointerdown(pointer(100, 100));
		swipe.onpointercancel();
		swipe.onpointerup(pointer(40, 100));

		expect(onNext).not.toHaveBeenCalled();
	});
});
