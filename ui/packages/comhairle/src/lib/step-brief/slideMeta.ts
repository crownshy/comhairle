import * as m from '$lib/paraglide/messages';
import { TOOL_META, type ToolType } from '$lib/tool_meta';

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
	required_votes?: number | null;
	follow_up_rounds_count?: number | null;
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

/**
 * Narrows a step's tool config to the fields the meta line reads.
 *
 * The generated `LocalizedToolConfig` is a wide union whose members share only `type`, so
 * this picks the two count fields structurally rather than switching on every member.
 */
export function toMetaToolConfig(config: unknown): MetaToolConfig | null {
	if (!config || typeof config !== 'object') return null;
	const record = config as Record<string, unknown>;
	return {
		type: typeof record.type === 'string' ? record.type : undefined,
		required_votes: typeof record.required_votes === 'number' ? record.required_votes : null,
		follow_up_rounds_count:
			typeof record.follow_up_rounds_count === 'number' ? record.follow_up_rounds_count : null
	};
}

/**
 * The cover's meta line, derived from real per-step config plus the tool's typical
 * duration (ADR-0017). Duration comes from the same `TOOL_META` map the design board's
 * "Estimated time" pill reads, so admin and participant cannot disagree.
 *
 * Tools with no meaningful count show duration alone.
 */
export function stepMeta(toolConfig: MetaToolConfig | null | undefined): StepMetaItem[] {
	const type = toolConfig?.type as ToolType | undefined;
	const meta = type ? TOOL_META[type] : undefined;
	const items: StepMetaItem[] = [];

	if (meta) {
		items.push({ kind: 'duration', label: minutes(meta.estimatedMinutes) });
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
