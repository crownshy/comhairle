/**
 * A conversation's moderation policy (ADR-0037): the reasons a moderator can pick when
 * rejecting a statement. It lives in the conversation's `metadata` jsonb, not on a step, so
 * every moderated tool in the conversation shares one list. Polis is the only reader today.
 *
 * A chosen label is still stored verbatim in `moderation_reason` as free text (ADR-0015), so
 * editing the list never rewrites reasons already recorded.
 */

/** The key the policy lives under in a conversation's `metadata` jsonb blob. */
export const MODERATION_POLICY_METADATA_KEY = 'moderation_policy';

export interface RejectReason {
	/** Short label shown in the reason picker and stored verbatim in `moderation_reason`. */
	label: string;
	/** What counts under this reason, shown to moderators when they pick it. */
	description?: string;
}

export interface ModerationPolicy {
	rejectReasons: RejectReason[];
	/** True when nothing is stored yet and the default list is standing in. */
	isDefault: boolean;
}

/**
 * The starting template for every conversation. "Multiple themes" is deliberately absent:
 * splitting is the correct action there, and the split flow rejects the original with its
 * own reason. "Illegal" is folded into "Harmful or abusive".
 */
export const DEFAULT_REJECT_REASONS: RejectReason[] = [
	{
		label: 'Off-topic or unclear',
		description: 'Not about this conversation, or too unclear for people to vote on.'
	},
	{
		label: 'Harmful or abusive',
		description: 'Hate speech, threats, harassment, or content that breaks the law.'
	},
	{
		label: 'Advertising or campaigning',
		description: 'Promotes a product, service, political party or campaign.'
	},
	{
		label: 'Privacy or personal info',
		description: 'Names or identifies a private person, or shares personal details.'
	},
	{
		label: 'Duplicate',
		description: 'Makes the same point as a statement that is already in the conversation.'
	}
];

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null && !Array.isArray(value);
}

/** Joins a reason's label to the moderator's note in `moderation_reason` (ADR-0015). */
export const REASON_NOTE_SEPARATOR = ': ';

export type RejectReasonLabelProblem = 'blank' | 'contains-separator' | 'duplicate';

/**
 * Why each label can't be saved, or null when it can, in input order. Labels are the picker
 * keys and the stored value, so a repeat (case-insensitive, first one wins) is dropped. A
 * label containing the separator couldn't be split back out of `moderation_reason` on export.
 */
export function rejectReasonLabelProblems(labels: string[]): (RejectReasonLabelProblem | null)[] {
	const seen = new Set<string>();
	return labels.map((label) => {
		const trimmed = label.trim();
		if (!trimmed) return 'blank';
		if (trimmed.includes(REASON_NOTE_SEPARATOR)) return 'contains-separator';
		const key = trimmed.toLowerCase();
		if (seen.has(key)) return 'duplicate';
		seen.add(key);
		return null;
	});
}

/** Trims every reason and drops the ones `rejectReasonLabelProblems` flags. */
export function cleanRejectReasons(reasons: RejectReason[]): RejectReason[] {
	const problems = rejectReasonLabelProblems(reasons.map((reason) => reason.label));
	const cleaned: RejectReason[] = [];
	reasons.forEach((reason, index) => {
		if (problems[index] !== null) return;
		const label = reason.label.trim();
		const description = reason.description?.trim();
		cleaned.push(description ? { label, description } : { label });
	});
	return cleaned;
}

/**
 * Reads the policy out of a conversation's `metadata` blob, never throwing. Nothing stored
 * means the default list. A stored empty list is kept as empty: the admin removed every
 * reason on purpose, and moderators then only get the free-text note.
 */
export function moderationPolicyFromMetadata(metadata: unknown): ModerationPolicy {
	const stored = isRecord(metadata) ? metadata[MODERATION_POLICY_METADATA_KEY] : undefined;
	if (!isRecord(stored) || !Array.isArray(stored.reject_reasons)) {
		return { rejectReasons: DEFAULT_REJECT_REASONS, isDefault: true };
	}

	const reasons: RejectReason[] = [];
	for (const raw of stored.reject_reasons) {
		if (!isRecord(raw) || typeof raw.label !== 'string') continue;
		reasons.push({
			label: raw.label,
			description: typeof raw.description === 'string' ? raw.description : undefined
		});
	}
	return { rejectReasons: cleanRejectReasons(reasons), isDefault: false };
}

/** The stored (snake_case) form written back into `metadata`. */
export function toStoredModerationPolicy(reasons: RejectReason[]) {
	return { reject_reasons: cleanRejectReasons(reasons) };
}

/**
 * Combine the chosen preset label and an optional free-text note into the single
 * string stored in `moderation_reason`. Returns undefined when neither is given,
 * so a reason-less reject stays reason-less.
 */
export function composeReason(label: string | null, note: string): string | undefined {
	const trimmed = note.trim();
	if (label && trimmed) return `${label}${REASON_NOTE_SEPARATOR}${trimmed}`;
	if (label) return label;
	return trimmed || undefined;
}

/**
 * The inverse of composeReason, for export: splits a stored `moderation_reason` back into the
 * label and the note, given the labels it could have come from. Anything that doesn't start
 * with a known label (a renamed label, the split flow's sentence, a note-only reject) comes
 * back whole as the note.
 */
export function splitReason(
	reason: string | null | undefined,
	labels: string[]
): { label: string; note: string } {
	const text = reason?.trim() ?? '';
	if (!text) return { label: '', note: '' };

	// Longest first, so a label that is a prefix of another can't claim its reasons.
	const candidates = [...labels].sort((a, b) => b.length - a.length);
	for (const label of candidates) {
		if (text === label) return { label, note: '' };
		const prefix = `${label}${REASON_NOTE_SEPARATOR}`;
		if (text.startsWith(prefix)) {
			return { label, note: text.slice(prefix.length).trim() };
		}
	}
	return { label: '', note: text };
}
