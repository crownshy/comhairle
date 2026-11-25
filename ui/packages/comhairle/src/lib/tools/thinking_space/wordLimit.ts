export const MAX_ANSWER_WORDS = 300;
export const MAX_SUMMARY_WORDS = 500;

export function countWords(text: string): number {
	const trimmed = text.trim();
	return trimmed ? trimmed.split(/\s+/).length : 0;
}

export function wordLimitIds(fieldId: string) {
	return {
		field: `${fieldId}-field`,
		wordCount: `${fieldId}-word-count`,
		overLimit: `${fieldId}-over-limit`
	};
}

/**
 * The `aria-describedby` value for a field with a word limit: the count while it is shown,
 * plus the over-limit message once the text is too long.
 */
export function wordLimitDescribedBy(
	fieldId: string,
	{ countShown, overLimit }: { countShown: boolean; overLimit: boolean }
): string | undefined {
	const ids = wordLimitIds(fieldId);
	const describedBy = [countShown && ids.wordCount, overLimit && ids.overLimit].filter(Boolean);
	return describedBy.length > 0 ? describedBy.join(' ') : undefined;
}
