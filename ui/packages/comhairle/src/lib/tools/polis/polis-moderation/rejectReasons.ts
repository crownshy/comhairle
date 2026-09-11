/**
 * Preset rejection reasons offered when a moderator rejects a Polis statement
 * (ADR-0015). Labels only, stored verbatim in `moderation_reason` as free text.
 *
 * "Multiple themes" is deliberately absent: splitting a statement is the correct
 * action there, and the split flow rejects the original with its own reason.
 * "Illegal" is folded into "Harmful or abusive"; the rare pure-illegal case goes
 * in the note. Keep this list short: it is a fast single tap, not a taxonomy.
 */
export const REJECT_REASONS = [
	'Off-topic or unclear',
	'Harmful or abusive',
	'Advertising or campaigning',
	'Privacy or personal info',
	'Duplicate'
] as const;

/**
 * Combine the chosen preset label and an optional free-text note into the single
 * string stored in `moderation_reason`. Returns undefined when neither is given,
 * so a reason-less reject stays reason-less.
 */
export function composeReason(label: string | null, note: string): string | undefined {
	const trimmed = note.trim();
	if (label && trimmed) return `${label}: ${trimmed}`;
	if (label) return label;
	return trimmed || undefined;
}
