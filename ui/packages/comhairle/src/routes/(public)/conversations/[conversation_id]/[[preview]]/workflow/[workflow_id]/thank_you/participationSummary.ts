// Below this many people a raw count reads as discouraging, so early participants get
// thanked for being early instead.
export const EARLY_PARTICIPANT_LIMIT = 15;

export type ParticipationSummary = { kind: 'early' } | { kind: 'count'; formattedCount: string };

/** What the count card shows: an early-participant note, or the count formatted for `locale`. */
export function participationSummary(
	participantCount: number,
	locale: string
): ParticipationSummary {
	if (participantCount < EARLY_PARTICIPANT_LIMIT) {
		return { kind: 'early' };
	}
	return { kind: 'count', formattedCount: participantCount.toLocaleString(locale) };
}
