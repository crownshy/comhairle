/**
 * Extra content for the engagement tools overview page (/admin/info/tools).
 * Each tool's name, "best for" and timings come from tool_guides.ts; this file adds
 * what participants do with it, so people can pick a tool by goal.
 */

export type ToolIcon = 'poll' | 'reflect' | 'learn' | 'video' | 'survey' | 'prioritise' | 'voice';

/** Keyed by the tool's key in TOOL_GUIDES. */
export const TOOL_OVERVIEW: Record<string, { icon: ToolIcon; goal: string }> = {
	polis: { icon: 'poll', goal: 'Share views and find common ground' },
	thinking_space: { icon: 'reflect', goal: 'Think a topic through on their own' },
	learn: { icon: 'learn', goal: 'Learn about the topic first' },
	survey: { icon: 'survey', goal: 'Answer set questions' },
	prioritization: { icon: 'prioritise', goal: 'Rank or rate proposals' },
	lived_experience: { icon: 'voice', goal: 'Share their own experience' },
	online_group_conversation: { icon: 'video', goal: 'Talk it through live' }
};

/**
 * The "Choose from templates" templates (conversation_templates.ts) list their steps by
 * display label. This maps each label to the engagement tool behind it, so the Handbook can
 * show the right icon and link. Labels with no tool (e.g. Workshop) are left out and shown
 * as in-person steps.
 */
export const TEMPLATE_STEP_TOOLS: Record<string, string> = {
	'Topic onboarding': 'learn',
	Survey: 'survey',
	'Wiki-poll (Pol.is)': 'polis',
	'Wiki Poll (Polis)': 'polis',
	'Online video conference': 'online_group_conversation',
	'Proposal prioritisation': 'prioritization',
	'Thinking Space': 'thinking_space'
};

/** Parses "10 to 15 minutes" into [10, 15]; undefined if there are no numbers. */
export function minutesRange(text: string | undefined): [number, number] | undefined {
	const numbers = (text ?? '').match(/\d+/g)?.map(Number);
	if (!numbers?.length) return undefined;
	return [numbers[0], numbers[1] ?? numbers[0]];
}
