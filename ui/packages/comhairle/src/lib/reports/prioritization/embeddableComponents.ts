import type { SectionBlockMeta } from '../embeds';

/**
 * The Prioritization section blocks a facilitator can embed into the report (ADR-0061). All of
 * them count or sum across reviewers; a proposal's free-text answers are never offered.
 */
export type PrioritizationEmbeddableComponentType =
	| 'prioritization-key-stats'
	| 'prioritization-ranking'
	| 'prioritization-proposal'
	| 'prioritization-themes';

export const PRIORITIZATION_EMBEDDABLE_COMPONENTS: SectionBlockMeta<PrioritizationEmbeddableComponentType>[] =
	[
		{
			type: 'prioritization-key-stats',
			label: 'Key stats',
			description: 'Reviewers, proposals and reviews handed in.'
		},
		{
			type: 'prioritization-ranking',
			label: 'Proposal ranking',
			description: 'Proposals ranked by their alignment score.'
		},
		{
			type: 'prioritization-proposal',
			label: 'Proposal results',
			description: 'How reviewers rated one proposal, question by question.',
			itemName: { one: 'proposal', other: 'proposals' }
		},
		{
			type: 'prioritization-themes',
			label: 'Themes',
			description: 'Themes the sensemaker found in the written answers.'
		}
	];
