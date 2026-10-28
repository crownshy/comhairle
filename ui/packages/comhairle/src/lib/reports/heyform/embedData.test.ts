import { describe, expect, it } from 'vitest';
import { scaleAnswers } from './embedData';

describe('scaleAnswers', () => {
	it('repeats each point once per person who picked it', () => {
		expect(
			scaleAnswers({
				max: 5,
				points: [
					{ point: 2, count: 1 },
					{ point: 4, count: 2 }
				]
			})
		).toEqual([2, 4, 4]);
	});
});
