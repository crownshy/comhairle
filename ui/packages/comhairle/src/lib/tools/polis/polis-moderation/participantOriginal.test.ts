import { describe, expect, it } from 'vitest';
import { participantOriginal } from './participantOriginal';

const participant = { id: 'a', original_statement_id: null };
const firstEdit = { id: 'b', original_statement_id: 'a' };
const secondEdit = { id: 'c', original_statement_id: 'b' };
const statements = [participant, firstEdit, secondEdit];

describe('participantOriginal', () => {
	it('returns an unedited statement as is', () => {
		expect(participantOriginal(participant, statements)).toBe(participant);
	});

	it('follows repeated edits back to what the participant wrote', () => {
		expect(participantOriginal(secondEdit, statements)).toBe(participant);
	});

	it('stops at the last statement it can find', () => {
		const orphan = { id: 'd', original_statement_id: 'missing' };
		expect(participantOriginal(orphan, [orphan])).toBe(orphan);
	});

	it('stops on a loop', () => {
		const looped = [
			{ id: 'x', original_statement_id: 'y' },
			{ id: 'y', original_statement_id: 'x' }
		];
		expect(['x', 'y']).toContain(participantOriginal(looped[0], looped).id);
	});
});
