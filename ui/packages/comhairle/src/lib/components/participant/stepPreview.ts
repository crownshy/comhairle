import { TOOL_META, type ToolType } from '$lib/tool_meta';
import type { Icon } from 'lucide-svelte';

/**
 * A workflow step as the conversation landing page shows it, before anyone has joined:
 * what it is called, what it looks like, and roughly how long it takes.
 *
 * Distinct from {@link import('./stepItems').StepItem}, which is about position and
 * progress inside a flow already under way.
 */
export type StepPreview = {
	id: string;
	/** The admin-authored step name, preferred over the verbose `TOOL_META.displayName`. */
	name: string;
	icon?: typeof Icon;
	minutes?: number;
	optional: boolean;
};

type RawStep = {
	id: string;
	name: string;
	required?: boolean;
	stepOrder?: number;
	toolConfig?: unknown;
	previewToolConfig?: unknown;
};

/**
 * A step's tool type, read from its live config and falling back to the draft one. An
 * unlaunched conversation has only the draft, which is what a preview shows.
 */
function toolTypeOf(step: RawStep): ToolType | undefined {
	const config = (step.toolConfig ?? step.previewToolConfig) as { type?: string } | null;
	const type = config?.type;
	return type && type in TOOL_META ? (type as ToolType) : undefined;
}

export function stepPreviews(steps: RawStep[] | null | undefined): StepPreview[] {
	if (!steps?.length) return [];
	return [...steps]
		.sort((a, b) => (a.stepOrder ?? 0) - (b.stepOrder ?? 0))
		.map((step) => {
			const type = toolTypeOf(step);
			const meta = type ? TOOL_META[type] : undefined;
			return {
				id: step.id,
				name: step.name,
				icon: meta?.icon,
				minutes: meta?.estimatedMinutes,
				optional: step.required === false
			};
		});
}

/**
 * Total estimated minutes across the steps.
 *
 * Each step contributes its tool's hardcoded `TOOL_META.estimatedMinutes`, the same figure
 * the admin design board quotes, because there is no per-step estimate in the data model
 * yet. Two conversations with the same tools therefore quote the same total however
 * differently they are configured. See CONTEXT.md, Estimated time.
 */
export function totalMinutes(previews: StepPreview[]): number {
	return previews.reduce((sum, s) => sum + (s.minutes ?? 0), 0);
}
