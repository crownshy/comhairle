import type { SectionBlockMeta } from '../embeds';

/**
 * The Thinking Space section blocks a facilitator can embed into the report (ADR-0061). Only
 * aggregates: a participant's answers and summary are shared with the organisers, never
 * with readers.
 */
export type ThinkingSpaceEmbeddableComponentType =
	'thinkingspace-key-stats' | 'thinkingspace-questions' | 'thinkingspace-themes';

export const THINKING_SPACE_EMBEDDABLE_COMPONENTS: SectionBlockMeta<ThinkingSpaceEmbeddableComponentType>[] =
	[
		{
			type: 'thinkingspace-key-stats',
			label: 'Key stats',
			description: 'People who shared a summary and how many follow-ups they took.'
		},
		{
			type: 'thinkingspace-questions',
			label: 'Opening questions',
			description: 'How many people answered each opening question.'
		},
		{
			type: 'thinkingspace-themes',
			label: 'Themes',
			description: 'Themes the sensemaker found across the shared summaries.'
		}
	];
