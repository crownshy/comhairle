import type { DemographicForm, DemographicQuestion } from './demographicPrototypeData';

/**
 * The CrownShy starter set: five core questions and one basic form that uses them.
 *
 * PLACEHOLDER CONTENT. The final five core questions and their consent wording are still to be
 * supplied, so the wording below is only here to make the empty-organisation flow demonstrable.
 * Replace the questions in `createStarterQuestions` and the form picks them up automatically.
 */

const question = (
	id: string,
	text: string,
	overrides: Partial<DemographicQuestion>
): DemographicQuestion => ({
	id,
	text,
	description: '',
	kind: 'single_choice',
	tags: ['Custom'],
	options: [],
	allowOther: false,
	preferNotToSay: true,
	maxSelections: null,
	placeholder: '',
	specialCategory: false,
	consentText: 'I consent to the collection of this information about me.',
	...overrides
});

export const createStarterQuestions = (): DemographicQuestion[] => [
	question('q-starter-age', 'What is your age?', { kind: 'number', tags: ['Age'] }),
	question('q-starter-gender', 'What is your gender?', {
		tags: ['Gender'],
		options: ['Woman', 'Man', 'Non-binary'],
		allowOther: true
	}),
	question('q-starter-ethnic-group', 'What is your ethnic group?', {
		tags: ['Ethnicity'],
		options: [
			'White',
			'Asian or Asian British',
			'Black, Black British or African',
			'Mixed or multiple'
		],
		allowOther: true,
		specialCategory: true,
		consentText: 'I consent to the collection of my ethnic group data.'
	}),
	question('q-starter-postcode', 'What is your postcode?', {
		kind: 'postcode',
		tags: ['Postcode']
	}),
	question('q-starter-health', 'Do you have a long-term health condition?', {
		options: ['Yes', 'No'],
		specialCategory: true,
		consentText: 'I consent to the collection of my health data.'
	})
];

export const STARTER_FORM_ID = 'f-starter';

export const createStarterForm = (): DemographicForm => {
	const questions = createStarterQuestions().map((q) => ({ questionId: q.id, required: false }));
	return {
		id: STARTER_FORM_ID,
		name: 'Starter demographic form',
		country: 'Scotland',
		status: 'published',
		version: 1,
		createdBy: 'CrownShy',
		editedLabel: 'Provided by CrownShy',
		usage: [],
		hasUnpublishedChanges: false,
		isNewlyCreated: false,
		questions,
		versions: [{ version: 1, label: 'Provided by CrownShy', questions }]
	};
};
