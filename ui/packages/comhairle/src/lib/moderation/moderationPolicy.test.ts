import { describe, expect, it } from 'vitest';
import type { ModerationPolicyDto, ToolConfigWithTranslations } from '@crownshy/api-client/api';
import {
	cleanRejectReasons,
	composeReason,
	policyStepUpdates,
	rejectReasonLabelProblems,
	rejectReasonsForStep,
	splitReason,
	withSavedReasonIds
} from './moderationPolicy';

function policy(id: string, labels: string[]): ModerationPolicyDto {
	return {
		id,
		conversationId: 'conversation',
		name: 'Moderation policy',
		createdAt: '2026-09-15T00:00:00Z',
		updatedAt: '2026-09-15T00:00:00Z',
		reasons: labels.map((label, position) => ({ id: `${id}-${position}`, label, position }))
	};
}

function polisConfig(moderationPolicyId: string | null): ToolConfigWithTranslations {
	return {
		type: 'polis',
		server_url: 'https://polis.example',
		poll_id: 'poll',
		admin_user: 'admin',
		admin_password: 'secret',
		required_votes: 10,
		show_remaining_statements: true,
		topic: null,
		description: null,
		is_active: true,
		strict_moderation: false,
		label_seeds_as_conversation_starter: false,
		moderation_policy_id: moderationPolicyId
	};
}

const defaults = [{ label: 'Duplicate', description: 'Same point again' }];

describe('rejectReasonsForStep', () => {
	const policies = [policy('first', ['Spam']), policy('second', ['Rude'])];

	it("uses the policy the step's config points at", () => {
		expect(rejectReasonsForStep(polisConfig('second'), policies, defaults)).toEqual([
			{ id: 'second-0', label: 'Rude' }
		]);
	});

	it("falls back to the conversation's first policy for a step with no or a missing policy", () => {
		expect(rejectReasonsForStep(polisConfig(null), policies, defaults)).toEqual([
			{ id: 'first-0', label: 'Spam' }
		]);
		expect(rejectReasonsForStep(polisConfig('deleted'), policies, defaults)).toEqual([
			{ id: 'first-0', label: 'Spam' }
		]);
	});

	it('uses the defaults when the conversation has no policy', () => {
		expect(rejectReasonsForStep(polisConfig(null), [], defaults)).toBe(defaults);
		expect(rejectReasonsForStep(null, [], defaults)).toBe(defaults);
	});
});

describe('policyStepUpdates', () => {
	it('points the preview and live Polis configs that point elsewhere, keeping other fields', () => {
		const updates = policyStepUpdates(
			[{ id: 'step', previewToolConfig: polisConfig(null), toolConfig: polisConfig('old') }],
			'new'
		);

		expect(updates).toEqual([
			{
				stepId: 'step',
				body: {
					preview_tool_config: polisConfig('new'),
					tool_config: polisConfig('new')
				}
			}
		]);
	});

	it('skips steps already pointing at the policy and non-Polis steps', () => {
		const learn = { type: 'learn', pages: [] } as unknown as ToolConfigWithTranslations;
		expect(
			policyStepUpdates(
				[
					{ id: 'pointed', previewToolConfig: polisConfig('new'), toolConfig: null },
					{ id: 'learn', previewToolConfig: learn, toolConfig: null }
				],
				'new'
			)
		).toEqual([]);
	});

	it('trusts where the page last pointed a step over the loaded config', () => {
		const steps = [{ id: 'step', previewToolConfig: polisConfig(null), toolConfig: null }];

		expect(policyStepUpdates(steps, 'new', new Map([['step', 'new']]))).toEqual([]);
		expect(policyStepUpdates(steps, null, new Map([['step', 'new']]))).toEqual([
			{ stepId: 'step', body: { preview_tool_config: polisConfig(null) } }
		]);
	});
});

describe('withSavedReasonIds', () => {
	it('gives new reasons the saved ids by label, ignoring case', () => {
		const saved = [
			{ id: 'a', label: 'Spam' },
			{ id: 'b', label: 'Rude' }
		];
		expect(withSavedReasonIds([{ label: 'spam ' }, { label: 'Rude' }], saved)).toEqual([
			{ id: 'a', label: 'spam ' },
			{ id: 'b', label: 'Rude' }
		]);
	});

	it("doesn't hand out an id another reason already holds, or match a renamed reason", () => {
		const saved = [{ id: 'a', label: 'Spam' }];
		expect(
			withSavedReasonIds(
				[{ id: 'a', label: 'Adverts' }, { label: 'Spam' }, { label: 'Rude' }],
				saved
			)
		).toEqual([{ id: 'a', label: 'Adverts' }, { label: 'Spam' }, { label: 'Rude' }]);
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

	it('keeps saved ids so a save updates reasons in place', () => {
		expect(cleanRejectReasons([{ id: 'a', label: ' Spam ', description: 'Ads' }])).toEqual([
			{ id: 'a', label: 'Spam', description: 'Ads' }
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
