/**
 * Mock follow-up question generator.
 *
 * For the prototype we don't hit RAGFlow. Instead we return a bank of
 * generic probing questions, lightly seeded by keywords in the user's
 * answer so the picker doesn't feel completely random.
 *
 * Replace with an array returned by RAGFlow once the agent template has
 * been updated to emit structured JSON (see THINKING_SPACE_TODO.md).
 */

const GENERIC_FOLLOWUPS = [
	'Can you tell me more about why you feel that way?',
	'What experiences have shaped this view?',
	'Who do you think is most affected by this?',
	'What would need to change for things to improve?',
	'How does this connect to other issues you care about?',
	'What concerns you most about this?',
	'What gives you hope on this topic?',
	'Are there trade-offs you find hardest to weigh?',
	'What would you want decision-makers to understand?',
	'How has your view on this changed over time?'
];

function pickN<T>(arr: T[], n: number): T[] {
	const pool = [...arr];
	const out: T[] = [];
	while (pool.length > 0 && out.length < n) {
		const idx = Math.floor(Math.random() * pool.length);
		out.push(pool.splice(idx, 1)[0]);
	}
	return out;
}

/**
 * Generate `count` follow-up question options for the picker.
 * `_priorAnswer` is currently unused but is wired through so we can later
 * shape the bank with simple keyword heuristics or a real LLM call.
 */
export function generateFollowUpOptions(
	_questionText: string,
	_priorAnswer: string,
	count = 5
): string[] {
	return pickN(GENERIC_FOLLOWUPS, Math.max(1, Math.min(count, GENERIC_FOLLOWUPS.length)));
}

/**
 * Mock claim extraction: truncate the participant's answer to a single
 * claim-sized sentence. Replace with the real claim emitted by the
 * RAGFlow opinion-extraction agent.
 */
export function extractMockClaim(answer: string): string {
	const trimmed = answer.trim().replace(/\s+/g, ' ');
	if (trimmed.length <= 180) return trimmed;
	return trimmed.slice(0, 177) + '…';
}
