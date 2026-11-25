import { describe, expect, it } from 'vitest';
import { activeSectionIndex } from './activeSection';

const TOPS = [0, 400, 900, 1500];
const position = (scrollTop: number) => ({ scrollTop, clientHeight: 600, scrollHeight: 2000 });

describe('activeSectionIndex', () => {
	it('starts on the first section', () => {
		expect(activeSectionIndex(TOPS, position(0))).toBe(0);
	});

	it('moves on once the next heading nears the top', () => {
		expect(activeSectionIndex(TOPS, position(250))).toBe(0);
		expect(activeSectionIndex(TOPS, position(350))).toBe(1);
		expect(activeSectionIndex(TOPS, position(900))).toBe(2);
	});

	it('picks the last section at the bottom, even if its heading never reached the top', () => {
		expect(activeSectionIndex(TOPS, position(1400))).toBe(3);
	});

	it('copes with no sections', () => {
		expect(activeSectionIndex([], position(0))).toBe(0);
	});
});
