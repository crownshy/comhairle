/**
 * The allow-list of Polis report components a facilitator can embed into the report
 * (see ADR-0012 and the "Embeddable section block" glossary entry). Section-level and
 * self-contained only — not the sub-primitives they compose from, and not the whole
 * `PolisInsights` page.
 */
export type PolisEmbeddableComponentType =
	| 'polis-key-stats'
	| 'polis-area-consensus'
	| 'polis-area-disagreement'
	| 'polis-consensus-continuum'
	| 'polis-opinion-groups';

export interface EmbeddableComponentMeta {
	type: PolisEmbeddableComponentType;
	label: string;
	description: string;
}

export const POLIS_EMBEDDABLE_COMPONENTS: EmbeddableComponentMeta[] = [
	{
		type: 'polis-key-stats',
		label: 'Key stats',
		description: 'Participants, statements and votes cast.'
	},
	{
		type: 'polis-area-consensus',
		label: 'Areas of consensus',
		description: 'Statements every opinion group agrees on.'
	},
	{
		type: 'polis-area-disagreement',
		label: 'Areas of disagreement',
		description: 'Statements the opinion groups split hardest on.'
	},
	{
		type: 'polis-consensus-continuum',
		label: 'Consensus continuum',
		description: 'Every statement plotted on a consensus-to-divisive axis (beeswarm).'
	},
	{
		type: 'polis-opinion-groups',
		label: 'Opinion groups',
		description: 'The emerging groups and their representative statements.'
	}
];
