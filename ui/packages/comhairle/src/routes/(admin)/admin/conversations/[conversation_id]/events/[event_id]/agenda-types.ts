export type AgendaItemType = 'standard' | 'breakout';

export interface BreakoutPrompt {
	title: string;
	instructions: string;
}

export interface AgendaItemData {
	id: string;
	type: AgendaItemType;
	title: string;
	// Breakout-specific fields
	duration?: number;
	groupSize?: number;
	prompts?: BreakoutPrompt[];
	assignmentMode?: string;
	balanceBy?: string[];
}
