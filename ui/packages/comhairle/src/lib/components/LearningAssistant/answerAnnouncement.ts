type AnsweredQuestion = {
	answer: string;
	streaming: boolean;
	error: string | null;
};

const CITATION_MARKER = /\s*\[ID:\d+\]/g;
const INLINE_MARKDOWN = /[*_`~]/g;

/** Strip citation markers and inline markdown so a screen reader reads plain sentences. */
export function toSpokenText(answer: string): string {
	return answer
		.replace(CITATION_MARKER, '')
		.replace(INLINE_MARKDOWN, '')
		.replace(/\s+/g, ' ')
		.trim();
}

/**
 * Text for the polite status region. It holds one fixed "finding" message while the answer
 * streams and switches to the whole answer when it finishes, so the region changes twice per
 * question instead of once per streamed token. Errors return nothing because the error box
 * announces itself as an alert.
 * @param newest The newest question and answer, or undefined when there are none.
 * @param askedThisVisit Whether the person asked or retried since the component mounted, so
 * loading earlier history stays silent.
 * @param findingText The localised "finding an answer" message.
 */
export function answerAnnouncement(
	newest: AnsweredQuestion | undefined,
	askedThisVisit: boolean,
	findingText: string
): string {
	if (!askedThisVisit || !newest || newest.error) return '';
	if (newest.streaming) return findingText;
	return toSpokenText(newest.answer);
}
