import { describe, expect, it } from 'vitest';
import { toVotedStatement } from './embedData';

describe('toVotedStatement', () => {
	it('maps a public statement onto the shape the report components read', () => {
		expect(
			toVotedStatement({
				tid: 1,
				text: 'More buses',
				themes: ['Transport'],
				overallVotes: { agrees: 3, disagrees: 1, passes: 0 },
				groupVotes: [{ groupId: 0, agrees: 2, disagrees: 1, passes: 0 }]
			})
		).toEqual({
			tid: 1,
			text: 'More buses',
			topics: ['Transport'],
			overall_votes: { agrees: 3, disagrees: 1, passes: 0 },
			group_votes: [{ group_id: 0, agrees: 2, disagrees: 1, passes: 0 }]
		});
	});
});
