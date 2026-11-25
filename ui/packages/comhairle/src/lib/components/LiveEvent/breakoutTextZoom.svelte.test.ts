import { describe, it, expect } from 'vitest';
import { BreakoutTextZoom } from './breakoutTextZoom.svelte';

describe('BreakoutTextZoom', () => {
	it('starts at normal size', () => {
		expect(new BreakoutTextZoom().zoomClass).toBe('[zoom:1]');
	});

	it('grows to the largest step and stops', () => {
		const zoom = new BreakoutTextZoom();
		zoom.grow();
		zoom.grow();
		zoom.grow();
		expect(zoom.zoomClass).toBe('[zoom:1.5]');
		expect(zoom.canGrow).toBe(false);
	});

	it('shrinks one step below normal and stops', () => {
		const zoom = new BreakoutTextZoom();
		zoom.shrink();
		zoom.shrink();
		expect(zoom.zoomClass).toBe('[zoom:0.875]');
		expect(zoom.canShrink).toBe(false);
	});
});
