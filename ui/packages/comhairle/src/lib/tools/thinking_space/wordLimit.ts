export const MAX_ANSWER_WORDS = 300;
export const MAX_SUMMARY_WORDS = 500;

export function countWords(text: string): number {
	const trimmed = text.trim();
	return trimmed ? trimmed.split(/\s+/).length : 0;
}
