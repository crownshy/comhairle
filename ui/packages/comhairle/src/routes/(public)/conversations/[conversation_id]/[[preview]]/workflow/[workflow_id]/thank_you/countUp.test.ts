import { describe, it, expect } from 'vitest';
import { countUpValue } from './countUp';

describe('countUpValue', () => {
	it('starts at zero and lands on the target', () => {
		expect(countUpValue(100, 0, 1000)).toBe(0);
		expect(countUpValue(100, 1000, 1000)).toBe(100);
		expect(countUpValue(100, 4000, 1000)).toBe(100);
	});

	it('eases out, so it is past halfway at the halfway point', () => {
		expect(countUpValue(100, 500, 1000)).toBeGreaterThan(50);
		expect(countUpValue(100, 500, 1000)).toBeLessThan(100);
	});

	it('jumps to the target when there is no time to animate in', () => {
		expect(countUpValue(42, 0, 0)).toBe(42);
	});
});
