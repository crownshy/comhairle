import type { ToolType } from '$lib/tool_meta';

const TOOLS_WITH_INSIGHTS: ToolType[] = ['polis', 'thinkingspace', 'prioritization', 'heyform'];

/** Whether a step's tool has an Insights page. Learn, Stories and the elicitation bot do not yet. */
export function hasInsights(toolType: string | undefined | null): boolean {
	return TOOLS_WITH_INSIGHTS.includes(toolType as ToolType);
}
