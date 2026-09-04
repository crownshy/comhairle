import { detectContentType } from '$lib/utils/contentDetection';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
import { TOOL_META, type ToolType } from '$lib/tool_meta';
import type { MetaToolConfig } from './slideMeta';

/**
 * How long a step is likely to take, derived from what the step actually asks of a
 * participant: the words on a Learn page, the statements a poll wants voted on, the
 * questions a thinking space opens with.
 *
 * The rates below are chosen, not measured. What they buy over the flat
 * `TOOL_META.estimatedMinutes` they replace is that two steps built on the same tool stop
 * quoting the same number: a one-paragraph brief and a twelve-page one now differ. A tool
 * whose config says nothing about length (a survey lives in HeyForm, not in our config)
 * keeps quoting its tool default.
 */

/** Silent reading, prose. Slower than the read-aloud pace `listen` promises. */
const READING_WORDS_PER_MINUTE = 200;

/** Reading someone else's statement and deciding agree / disagree / pass. */
const SECONDS_PER_VOTE = 15;

/** Reading a poll's own framing, and writing a statement of your own at the end. */
const POLIS_OVERHEAD_SECONDS = 90;

/** Writing an answer in your own words, whether a root question or a follow-up. */
const SECONDS_PER_WRITTEN_ANSWER = 90;

/** Scoring one proposal against one question. */
const SECONDS_PER_SCORE = 25;

/** Reading the proposals before scoring any of them. */
const PRIORITIZATION_OVERHEAD_SECONDS = 60;

/** Watching one recording. */
const SECONDS_PER_STORY = 90;

/** Thinking about, then recording, one of your own. */
const STORIES_RECORDING_SECONDS = 180;

/**
 * Plain text out of stored rich text, for counting only.
 *
 * ProseMirror JSON is walked for its `text` nodes; anything else is treated as Markdown
 * with its syntax knocked out, so `## Heading` counts as one word rather than two. A
 * hyphen only goes if it stands alone as a bullet: `well-being` is one word.
 */
function plainText(content: string | null | undefined): string {
	const detected = detectContentType(content);
	if (detected.type === CONTENT_TYPES.JSON) return proseMirrorText(detected.content);
	return String(detected.content ?? '')
		.replace(/[#*_`~>|[\]()]/g, ' ')
		.replace(/(^|\s)[-+]+(?=\s|$)/gm, ' ');
}

function proseMirrorText(node: unknown): string {
	if (Array.isArray(node)) return node.map(proseMirrorText).join(' ');
	if (!node || typeof node !== 'object') return '';
	const record = node as { text?: unknown; content?: unknown };
	const own = typeof record.text === 'string' ? record.text : '';
	return record.content ? `${own} ${proseMirrorText(record.content)}` : own;
}

export function countWords(content: string | null | undefined): number {
	return plainText(content).split(/\s+/).filter(Boolean).length;
}

/**
 * Words on each Learn page, in the language the reader is in.
 *
 * A page arrives as one entry per translation. A page with no entry for this locale falls
 * back to the first one there is: the participant will be reading roughly that much text
 * whichever translation they are served. Pages that are still a `text_content_id`
 * reference carry no words and contribute nothing.
 */
export function learnPageWords(pages: unknown, locale: string): number[] {
	if (!Array.isArray(pages)) return [];
	return pages.map((page) => {
		if (!Array.isArray(page)) return 0;
		const translations = page as Array<{ lang?: unknown; content?: unknown }>;
		const match = translations.find((entry) => entry.lang === locale) ?? translations[0];
		return typeof match?.content === 'string' ? countWords(match.content) : 0;
	});
}

/**
 * Seconds the step's own config accounts for, or null when the config says nothing about
 * length and the caller should fall back to the tool's default.
 */
function derivedSeconds(config: MetaToolConfig, type: ToolType): number | null {
	switch (type) {
		case 'learn': {
			const words = (config.page_words ?? []).reduce((sum, count) => sum + count, 0);
			return words > 0 ? (words / READING_WORDS_PER_MINUTE) * 60 : null;
		}
		case 'polis': {
			const votes = config.required_votes ?? 0;
			return votes > 0 ? votes * SECONDS_PER_VOTE + POLIS_OVERHEAD_SECONDS : null;
		}
		case 'thinkingspace': {
			const roots = config.root_question_count ?? 0;
			const rounds = config.follow_up_rounds_count ?? 0;
			return roots > 0 ? roots * (1 + rounds) * SECONDS_PER_WRITTEN_ANSWER : null;
		}
		case 'prioritization': {
			const questions = config.question_count ?? 0;
			const reviews = config.required_reviews ?? 0;
			if (questions < 1 || reviews < 1) return null;
			return reviews * questions * SECONDS_PER_SCORE + PRIORITIZATION_OVERHEAD_SECONDS;
		}
		case 'stories': {
			const toSee = config.to_see ?? 0;
			return toSee > 0 ? toSee * SECONDS_PER_STORY + STORIES_RECORDING_SECONDS : null;
		}
		// A survey's questions live in HeyForm and an elicitation bot's script is not fixed,
		// so neither config can say how long it runs.
		default:
			return null;
	}
}

/**
 * Whole minutes to quote for a step, or null when the tool is unknown. Never zero: a step
 * that exists takes a moment, and "0 minutes" reads as a bug.
 */
export function estimateMinutes(config: MetaToolConfig | null | undefined): number | null {
	const type = config?.type as ToolType | undefined;
	const meta = type ? TOOL_META[type] : undefined;
	if (!config || !type || !meta) return null;

	const seconds = derivedSeconds(config, type);
	if (seconds === null) return meta.estimatedMinutes;
	return Math.max(1, Math.round(seconds / 60));
}
