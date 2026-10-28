import type { SectionBlockMeta } from '../embeds';

/**
 * HeyForm's one section block, embedded once per question (ADR-0061). Only choice, rating and
 * opinion scale questions can go public; the API lists them for the add-chart dialog.
 */
export const HEYFORM_EMBEDDABLE_COMPONENTS: SectionBlockMeta<'heyform-question'>[] = [
	{
		type: 'heyform-question',
		label: 'Question results',
		description: 'How people answered one question.',
		itemName: { one: 'question', other: 'questions' }
	}
];
