export type QuestionKind =
	| 'single_choice'
	| 'multiple_choice'
	| 'number'
	| 'date'
	| 'open_text'
	| 'postcode';

export const QUESTION_KIND_LABELS: Record<QuestionKind, string> = {
	single_choice: 'Single choice',
	multiple_choice: 'Multiple choice',
	number: 'Number',
	date: 'Date (YYYY/MM/DD)',
	open_text: 'Open text',
	postcode: 'Postcode, by region'
};

const DEFAULT_PLACEHOLDERS: Record<QuestionKind, string> = {
	single_choice: '',
	multiple_choice: '',
	number: 'Enter a number',
	date: 'YYYY/MM/DD',
	postcode: 'Enter your postcode',
	open_text: 'Type your answer'
};

export const defaultPlaceholder = (kind: QuestionKind) => DEFAULT_PLACEHOLDERS[kind];

export const TAG_OPTIONS = ['Age', 'Gender', 'Ethnicity', 'Postcode', 'Custom'];
export const COUNTRY_OPTIONS = [
	'Scotland',
	'England',
	'Wales',
	'Northern Ireland',
	'United States'
];
export const CURRENT_USER = 'Shu';

export type DemographicQuestion = {
	id: string;
	text: string;
	description: string;
	kind: QuestionKind;
	tags: string[];
	options: string[];
	allowOther: boolean;
	preferNotToSay: boolean;
	placeholder: string;
	specialCategory: boolean;
};

export type FormQuestion = { questionId: string; required: boolean };

export type DemographicForm = {
	id: string;
	name: string;
	country: string;
	status: 'draft' | 'published';
	version: number;
	createdBy: string;
	editedLabel: string;
	usedInConversations: number;
	isNewlyCreated: boolean;
	questions: FormQuestion[];
};

export const isChoiceKind = (kind: QuestionKind) =>
	kind === 'single_choice' || kind === 'multiple_choice';

const newId = (prefix: string) => `${prefix}-${Math.random().toString(36).slice(2, 8)}`;

export const blankQuestion = (): DemographicQuestion => ({
	id: newId('q'),
	text: '',
	description: '',
	kind: 'single_choice',
	tags: ['Custom'],
	options: ['', '', ''],
	allowOther: false,
	preferNotToSay: true,
	placeholder: '',
	specialCategory: false
});

export const blankForm = (): DemographicForm => ({
	id: newId('f'),
	name: '',
	country: 'Scotland',
	status: 'draft',
	version: 0,
	createdBy: CURRENT_USER,
	editedLabel: 'Not yet published',
	usedInConversations: 0,
	isNewlyCreated: false,
	questions: []
});

export const usedInFormsCount = (questionId: string, forms: DemographicForm[]) =>
	forms.filter((form) => form.questions.some((q) => q.questionId === questionId)).length;

export const specialCategoryCount = (form: DemographicForm, questions: DemographicQuestion[]) =>
	form.questions.filter((fq) => questions.find((q) => q.id === fq.questionId)?.specialCategory)
		.length;

const choice = (
	id: string,
	text: string,
	tags: string[],
	options: string[],
	extra: Partial<DemographicQuestion> = {}
): DemographicQuestion => ({
	id,
	text,
	description: '',
	kind: 'single_choice',
	tags,
	options,
	allowOther: false,
	preferNotToSay: true,
	placeholder: '',
	specialCategory: false,
	...extra
});

export const createInitialQuestions = (): DemographicQuestion[] => [
	choice('q-age', 'What is your age?', ['Age'], [], { kind: 'number' }),
	choice('q-gender', 'What is your gender?', ['Gender'], ['Woman', 'Man', 'Non-binary'], {
		allowOther: true
	}),
	choice('q-birth-year', 'What is your year of birth?', ['Age'], [], { kind: 'date' }),
	choice('q-birthday', 'What is your birthday?', ['Age'], [], { kind: 'date' }),
	choice(
		'q-ethnic-group',
		'What is your ethnic group?',
		['Ethnicity'],
		['White', 'Asian or Asian British', 'Black, Black British or African', 'Mixed or multiple'],
		{ allowOther: true, specialCategory: true }
	),
	choice('q-postcode', 'What is your postcode?', ['Postcode'], [], { kind: 'postcode' }),
	choice(
		'q-employment',
		'Employment status',
		['Custom'],
		['Employed full time', 'Employed part time', 'Self-employed', 'Student', 'Retired']
	),
	choice('q-housing', 'Do you rent or own your home?', ['Custom'], ['Rent', 'Own', 'Other']),
	choice(
		'q-caring',
		'Caring responsibilities',
		['Custom'],
		['Child under 16', 'Adult family member', 'No caring responsibilities'],
		{ kind: 'multiple_choice', specialCategory: true }
	)
];

const formQuestions = (ids: string[], required: string[] = []): FormQuestion[] =>
	ids.map((questionId) => ({ questionId, required: required.includes(questionId) }));

export const createInitialForms = (): DemographicForm[] => [
	{
		id: 'f-standard',
		name: 'Standard Demographic form',
		country: 'Scotland',
		status: 'published',
		version: 1,
		createdBy: 'Shu',
		editedLabel: 'Edited 28 Sep by Kimi',
		usedInConversations: 4,
		isNewlyCreated: false,
		questions: formQuestions(['q-age', 'q-gender', 'q-birthday', 'q-postcode'], ['q-age'])
	},
	{
		id: 'f-social-prescribing',
		name: 'Social Prescribing team form',
		country: 'Scotland',
		status: 'published',
		version: 2,
		createdBy: 'Kimi',
		editedLabel: 'Edited 2 Oct by Kimi',
		usedInConversations: 1,
		isNewlyCreated: false,
		questions: formQuestions([
			'q-age',
			'q-gender',
			'q-ethnic-group',
			'q-postcode',
			'q-employment',
			'q-housing',
			'q-caring'
		])
	},
	{
		id: 'f-youth',
		name: 'Youth panel (16+)',
		country: 'Scotland',
		status: 'draft',
		version: 0,
		createdBy: 'Shu',
		editedLabel: 'Not yet published',
		usedInConversations: 0,
		isNewlyCreated: false,
		questions: formQuestions(['q-age', 'q-gender', 'q-postcode', 'q-employment'])
	}
];
