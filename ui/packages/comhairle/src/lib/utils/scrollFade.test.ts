import { describe, it, expect } from 'vitest';
import { scrollFadeMask } from './scrollFade';

describe('scrollFadeMask', () => {
	it('has no mask when everything fits', () => {
		expect(scrollFadeMask({ scrollTop: 0, scrollHeight: 300, clientHeight: 300 })).toBe('none');
	});

	it('fades only the bottom at the top of long content', () => {
		expect(scrollFadeMask({ scrollTop: 0, scrollHeight: 900, clientHeight: 300 })).toBe(
			'linear-gradient(to bottom, black, black, black calc(100% - 2rem), transparent)'
		);
	});

	it('fades both edges in the middle', () => {
		expect(scrollFadeMask({ scrollTop: 200, scrollHeight: 900, clientHeight: 300 })).toBe(
			'linear-gradient(to bottom, transparent, black 2rem, black calc(100% - 2rem), transparent)'
		);
	});

	it('fades only the top at the end, so the last line stays clear', () => {
		expect(scrollFadeMask({ scrollTop: 600, scrollHeight: 900, clientHeight: 300 })).toBe(
			'linear-gradient(to bottom, transparent, black 2rem, black)'
		);
	});

	it('treats a sub-pixel gap at the end as the end', () => {
		expect(scrollFadeMask({ scrollTop: 599.5, scrollHeight: 900, clientHeight: 300 })).toBe(
			'linear-gradient(to bottom, transparent, black 2rem, black)'
		);
	});
});
