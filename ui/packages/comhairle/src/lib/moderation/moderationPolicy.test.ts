import { describe, expect, it } from 'vitest';
import {
	DEFAULT_REJECT_REASONS,
	MODERATION_POLICY_METADATA_KEY,
	cleanRejectReasons,
	composeReason,
	moderationPolicyFromMetadata,
	rejectReasonLabelProblems,
	splitReason,
	toStoredModerationPolicy
} from './moderationPolicy';

describe('moderationPolicyFromMetadata', () => {
	it('falls back to the default list when nothing is stored', () => {
		expect(moderationPolicyFromMetadata(null)).toEqual({
			rejectReasons: DEFAULT_REJECT_REASONS,
			isDefault: true
		});
		expect(moderationPolicyFromMetadata({ glossary: [] }).isDefault).toBe(true);
		expect(
			moderationPolicyFromMetadata({ [MODERATION_POLICY_METADATA_KEY]: 'nonsense' }).isDefault
		).toBe(true);
	});

	it('reads stored reasons and drops malformed entries', () => {
		const policy = moderationPolicyFromMetadata({
			[MODERATION_POLICY_METADATA_KEY]: {
				reject_reasons: [
					{ label: 'Spam', description: 'Links to shops' },
					{ label: 42 },
					'Duplicate',
					{ label: '  Rude  ', description: 7 }
				]
			}
		});
		expect(policy).toEqual({
			rejectReasons: [{ label: 'Spam', description: 'Links to shops' }, { label: 'Rude' }],
			isDefault: false
		});
	});

	it('keeps a stored empty list empty instead of restoring the defaults', () => {
		expect(
			moderationPolicyFromMetadata({
				[MODERATION_POLICY_METADATA_KEY]: { reject_reasons: [] }
			})
		).toEqual({ rejectReasons: [], isDefault: false });
	});
});

describe('cleanRejectReasons', () => {
	it('trims, drops blank labels and keeps the first of a repeated label', () => {
		expect(
			cleanRejectReasons([
				{ label: ' Spam ', description: '  ' },
				{ label: '' },
				{ label: 'spam', description: 'second copy' },
				{ label: 'Duplicate', description: ' Same point ' }
			])
		).toEqual([{ label: 'Spam' }, { label: 'Duplicate', description: 'Same point' }]);
	});

	it('drops a label containing the reason/note separator', () => {
		expect(cleanRejectReasons([{ label: 'Spam: bots' }, { label: 'Spam:bots' }])).toEqual([
			{ label: 'Spam:bots' }
		]);
	});
});

describe('rejectReasonLabelProblems', () => {
	it('flags blank, separator and repeated labels in input order', () => {
		expect(
			rejectReasonLabelProblems(['Spam', '  ', 'Spam: bots', ' spam ', 'Duplicate'])
		).toEqual([null, 'blank', 'contains-separator', 'duplicate', null]);
	});

	it('does not let a separator label claim its key from a later valid label', () => {
		expect(rejectReasonLabelProblems(['Spam: bots', 'spam: bots', 'Spam'])).toEqual([
			'contains-separator',
			'contains-separator',
			null
		]);
	});
});

describe('toStoredModerationPolicy', () => {
	it('writes the snake_case shape moderationPolicyFromMetadata reads', () => {
		const stored = toStoredModerationPolicy([{ label: 'Spam', description: 'Ads' }]);
		expect(stored).toEqual({ reject_reasons: [{ label: 'Spam', description: 'Ads' }] });
		expect(
			moderationPolicyFromMetadata({ [MODERATION_POLICY_METADATA_KEY]: stored }).rejectReasons
		).toEqual([{ label: 'Spam', description: 'Ads' }]);
	});
});

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

describe('splitReason', () => {
	const labels = ['Duplicate', 'Duplicate of a seed', 'Spam'];

	it('undoes composeReason for a label with a note', () => {
		expect(splitReason(composeReason('Duplicate', 'same as #12'), labels)).toEqual({
			label: 'Duplicate',
			note: 'same as #12'
		});
	});

	it('matches a bare label', () => {
		expect(splitReason('Spam', labels)).toEqual({ label: 'Spam', note: '' });
	});

	it('prefers the longest label when one label is a prefix of another', () => {
		expect(splitReason('Duplicate of a seed: #3', labels)).toEqual({
			label: 'Duplicate of a seed',
			note: '#3'
		});
	});

	it('returns unknown text whole as the note', () => {
		expect(splitReason('Reworded/split by moderator into 2 statement(s)', labels)).toEqual({
			label: '',
			note: 'Reworded/split by moderator into 2 statement(s)'
		});
		expect(splitReason('Spammy: not a label', labels)).toEqual({
			label: '',
			note: 'Spammy: not a label'
		});
	});

	it('returns empty parts for a missing reason', () => {
		expect(splitReason(null, labels)).toEqual({ label: '', note: '' });
		expect(splitReason(undefined, labels)).toEqual({ label: '', note: '' });
	});
});
