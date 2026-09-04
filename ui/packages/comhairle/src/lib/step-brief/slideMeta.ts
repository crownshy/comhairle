import * as m from '$lib/paraglide/messages';
import { getLocale } from '$lib/paraglide/runtime.js';
import { TOOL_META, type ToolType } from '$lib/tool_meta';
import { estimateMinutes, learnPageWords } from './stepDuration';

export type StepMetaItem = {
	/** Lucide icon name resolved by the cover, kept as data so this stays testable. */
	kind: 'duration' | 'count';
	label: string;
};

/**
 * The subset of a step's tool config the meta line reads. Deliberately structural rather
 * than the generated union, so this stays a pure function over plain data.
 */
export type MetaToolConfig = {
	type?: string;
	/** polis: statements the participant is asked to vote on. */
	required_votes?: number | null;
	/** thinkingspace: rounds of follow-ups after each root question. */
	follow_up_rounds_count?: number | null;
	/** learn: words on each page, already resolved to the reader's language. */
	page_words?: number[];
	/** thinkingspace: questions the space opens with. */
	root_question_count?: number;
	/** prioritization: questions asked about each proposal. */
	question_count?: number;
	/** prioritization: proposals the participant is asked to score. */
	required_reviews?: number | null;
	/** stories: recordings the participant is asked to watch. */
	to_see?: number | null;
};

function minutes(count: number): string {
	return count === 1 ? m.step_meta_minute({ count }) : m.step_meta_minutes({ count });
}

function opinions(count: number): string {
	return count === 1 ? m.step_meta_opinion({ count }) : m.step_meta_opinions({ count });
}

function followUps(count: number): string {
	return count === 1
		? m.step_meta_follow_up_question({ count })
		: m.step_meta_follow_up_questions({ count });
}

function numberOrNull(value: unknown): number | null {
	return typeof value === 'number' ? value : null;
}

function countOf(value: unknown): number {
	return Array.isArray(value) ? value.length : 0;
}

/**
 * Narrows a step's tool config to the fields the meta line and the duration estimate read.
 *
 * The generated `LocalizedToolConfig` is a wide union whose members share only `type`, so
 * this picks what it needs structurally rather than switching on every member. Learn pages
 * are reduced to a word count here, while the locale is still in hand, so everything
 * downstream is plain numbers.
 */
export function toMetaToolConfig(
	config: unknown,
	locale: string = getLocale()
): MetaToolConfig | null {
	if (!config || typeof config !== 'object') return null;
	const record = config as Record<string, unknown>;
	return {
		type: typeof record.type === 'string' ? record.type : undefined,
		required_votes: numberOrNull(record.required_votes),
		follow_up_rounds_count: numberOrNull(record.follow_up_rounds_count),
		page_words: learnPageWords(record.pages, locale),
		root_question_count: countOf(record.root_questions),
		question_count: countOf(record.questions) + countOf(record.section_questions),
		required_reviews: numberOrNull(record.required_reviews),
		to_see: numberOrNull(record.to_see)
	};
}

/**
 * The cover's meta line, derived from real per-step config (ADR-0017): how long the step
 * looks like it will take, then the counts worth knowing before starting it.
 *
 * Duration comes from {@link estimateMinutes}, which the design board's "Estimated time"
 * pill reads too, so admin and participant cannot disagree.
 */
export function stepMeta(toolConfig: MetaToolConfig | null | undefined): StepMetaItem[] {
	const type = toolConfig?.type as ToolType | undefined;
	const meta = type ? TOOL_META[type] : undefined;
	const items: StepMetaItem[] = [];

	const estimate = estimateMinutes(toolConfig);
	if (meta && estimate !== null) {
		items.push({ kind: 'duration', label: minutes(estimate) });
	}

	const votes = toolConfig?.required_votes;
	if (type === 'polis' && typeof votes === 'number' && votes > 0) {
		items.push({ kind: 'count', label: opinions(votes) });
	}

	const rounds = toolConfig?.follow_up_rounds_count;
	if (type === 'thinkingspace' && typeof rounds === 'number' && rounds > 0) {
		items.push({ kind: 'count', label: followUps(rounds) });
	}

	return items;
}
