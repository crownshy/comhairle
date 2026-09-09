import { describe, it, expect } from 'vitest';
import { render } from 'svelte/server';
import StepProgressBar from './StepProgressBar.svelte';
import type { StepItem } from './stepItems';

const intro: StepItem = {
	id: 'landing',
	name: 'Before you start',
	status: 'completed',
	isIntro: true
};
const steps: StepItem[] = [
	{ id: 's1', name: 'Learn', status: 'completed', href: '/s1' },
	{ id: 's2', name: 'Vote', status: 'completed', href: '/s2' }
];
const outro: StepItem = { id: 'thank-you', name: 'Thank you', status: 'current', isOutro: true };

const segmentCount = (body: string) => (body.match(/class="[^"]*h-2 /g) ?? []).length;

describe('StepProgressBar with an outro item', () => {
	it('draws no segment for the thank-you screen, so "Step N of M" and the bar agree on M', () => {
		const without = render(StepProgressBar, {
			props: { steps: [intro, ...steps], currentIndex: 2, fill: 1 }
		}).body;
		const withOutro = render(StepProgressBar, {
			props: { steps: [intro, ...steps, outro], currentIndex: 2, fill: 1 }
		}).body;

		expect(segmentCount(without)).toBe(3);
		expect(segmentCount(withOutro)).toBe(3);
	});

	it('keeps the last step as the full track when the outro trails it', () => {
		const { body } = render(StepProgressBar, {
			props: { steps: [intro, ...steps, outro], currentIndex: 2, fill: 1 }
		});
		// One flexible track, filled all the way.
		expect(body.match(/flex-1/g)?.length).toBe(1);
		expect(body).toContain('width: 100%');
	});
});
