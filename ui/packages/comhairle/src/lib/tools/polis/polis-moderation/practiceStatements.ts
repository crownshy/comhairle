import type { PolisStatementAux } from '@crownshy/api-client/api';
import { splitReason, type RejectReason } from '$lib/moderation/moderationPolicy';
import type { ModerationStatus } from './moderationStatus';

type Decision = Exclude<ModerationStatus, 'pending'>;

type PracticeStatement = {
	text: string;
	expected: Decision;
	/** Said when the moderator makes the expected call. */
	right: string;
	/** Said otherwise, and names the right call. */
	wrong: string;
	/** For a reject, matches the policy reasons that fit it. Any reason is fine when none do. */
	fittingReason?: RegExp;
};

const PRACTICE_STATEMENTS: PracticeStatement[] = [
	{
		text: 'The organisers have ignored the queues for years. It’s a disgrace and they should be embarrassed.',
		expected: 'accepted',
		right: 'Blunt criticism of the organisers is still a clear view people can vote on. Accept it as written.',
		wrong: 'Criticism of the organisers is a view people can vote on, even when it’s blunt. Accept it as written.'
	},
	{
		text: 'Dave Morris who runs the dodgems is always rude to kids. Someone should sack him.',
		expected: 'rejected',
		right: 'It names a private person and calls for them to lose their job. Reject it for privacy.',
		wrong: 'It names a private person and calls for them to lose their job. Reject it for privacy, however fair the complaint is.',
		fittingReason: /privacy|personal/i
	},
	{
		text: 'Get 50% off your entry at cheap-park-tickets.biz, today only!',
		expected: 'rejected',
		right: 'It’s an advert, not a view. Reject it as advertising.',
		wrong: 'It’s an advert, not a view, so there is nothing to vote on. Reject it as advertising.',
		fittingReason: /advert|spam|promot/i
	}
];

// Practice rows never reach the API, so fixed placeholder values fill the fields it would set.
const PLACEHOLDER_UUID = '00000000-0000-0000-0000-000000000000';
const PLACEHOLDER_DATE = '2026-01-01T00:00:00Z';

/** Made-up statements, waiting, in the same shape as the real moderation list. */
export function practiceRows(): PolisStatementAux[] {
	return PRACTICE_STATEMENTS.map((statement, index) => ({
		id: `practice-${index + 1}`,
		polis_statement_id: index + 1,
		statement_text: statement.text,
		moderation_status: 'pending',
		moderation_reason: null,
		is_seed: false,
		themes: [],
		polis_conversation_id: 'practice',
		workflow_step_id: PLACEHOLDER_UUID,
		zid: 0,
		created_at: PLACEHOLDER_DATE,
		updated_at: PLACEHOLDER_DATE
	}));
}

/** Sets the status of the given rows, keeping the reason only on a reject, as the API does. */
export function moderatePracticeRows(
	rows: PolisStatementAux[],
	ids: string[],
	status: ModerationStatus,
	reason?: string
): PolisStatementAux[] {
	const targets = new Set(ids);
	return rows.map((row) =>
		targets.has(row.id)
			? {
					...row,
					moderation_status: status,
					moderation_reason: status === 'rejected' ? (reason ?? null) : null
				}
			: row
	);
}

export type PracticeFeedback = { id: string; number: number; correct: boolean; message: string };

/** The policy reasons that fit a practice reject, or null when any reason will do. */
function fittingReasons(statement: PracticeStatement, rejectReasons: RejectReason[]) {
	const pattern = statement.fittingReason;
	if (!pattern) return null;
	const fitting = rejectReasons.filter((reason) => pattern.test(reason.label));
	return fitting.length > 0 ? fitting : null;
}

function feedbackMessage(
	row: PolisStatementAux,
	statement: PracticeStatement,
	rejectReasons: RejectReason[]
): { correct: boolean; message: string } {
	if (row.moderation_status !== statement.expected)
		return { correct: false, message: `Not quite. ${statement.wrong}` };

	const fitting =
		row.moderation_status === 'rejected' && fittingReasons(statement, rejectReasons);
	if (fitting) {
		const labels = rejectReasons.map((reason) => reason.label);
		const picked = splitReason(row.moderation_reason, labels).label;
		if (!fitting.some((reason) => reason.label === picked))
			return {
				correct: false,
				message: `Right to reject, but the reason should be ${fitting[0].label}. ${statement.right}`
			};
	}
	return { correct: true, message: `Right. ${statement.right}` };
}

/** Whether each decided practice row was the right call, and why. Waiting rows have none. */
export function practiceFeedback(
	rows: PolisStatementAux[],
	rejectReasons: RejectReason[]
): PracticeFeedback[] {
	return rows.flatMap((row) => {
		const statement = PRACTICE_STATEMENTS[row.polis_statement_id - 1];
		if (!statement || row.moderation_status === 'pending') return [];
		return [
			{
				id: row.id,
				number: row.polis_statement_id,
				...feedbackMessage(row, statement, rejectReasons)
			}
		];
	});
}
