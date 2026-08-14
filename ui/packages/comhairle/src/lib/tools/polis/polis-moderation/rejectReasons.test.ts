import { describe, expect, it } from 'vitest';
import { composeReason } from './rejectReasons';

describe('composeReason', () => {
	it('combines a label and a note', () => {
		expect(composeReason('Duplicate', 'same as #12')).toBe('Duplicate: same as #12');
	});

	it('returns the label alone when there is no note', () => {
		expect(composeReason('Off-topic or unclear', '')).toBe('Off-topic or unclear');
		expect(composeReason('Off-topic or unclear', '   ')).toBe('Off-topic or unclear');
	});

	it('returns the trimmed note alone when no label is chosen', () => {
		expect(composeReason(null, '  spam  ')).toBe('spam');
	});

	it('returns undefined when neither is given, so a reason-less reject stays reason-less', () => {
		expect(composeReason(null, '')).toBeUndefined();
		expect(composeReason(null, '   ')).toBeUndefined();
	});
});
