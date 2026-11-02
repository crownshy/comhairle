/** The kinds of impact the API stores, as free strings. */
export const IMPACT_KINDS = [
	{ value: 'policy', label: 'Policy' },
	{ value: 'debate', label: 'Debate' },
	{ value: 'followup_conversation', label: 'Follow-up conversation' }
] as const;

export function impactKindLabel(kind: string): string {
	return IMPACT_KINDS.find((known) => known.value === kind)?.label ?? kind;
}
