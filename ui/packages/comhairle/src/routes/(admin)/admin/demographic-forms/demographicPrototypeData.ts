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

export const selectionHint = (kind: QuestionKind, maxSelections: number | null) => {
	if (kind !== 'multiple_choice') return '';
	if (maxSelections === null) return 'Select all that apply';
	return maxSelections === 1 ? 'Select one' : `Select up to ${maxSelections}`;
};

export const defaultPlaceholder = (kind: QuestionKind) => DEFAULT_PLACEHOLDERS[kind];

const TAG_COLOR_CLASSES: Record<string, string> = {
	Age: 'bg-blue-100 text-blue-900 outline-blue-300 dark:bg-blue-950 dark:text-blue-100',
	Gender: 'bg-purple-100 text-purple-900 outline-purple-300 dark:bg-purple-950 dark:text-purple-100',
	Ethnicity:
		'bg-amber-100 text-amber-900 outline-amber-300 dark:bg-amber-950 dark:text-amber-100',
	Postcode: 'bg-green-100 text-green-900 outline-green-300 dark:bg-green-950 dark:text-green-100',
	Custom: 'bg-slate-100 text-slate-800 outline-slate-300 dark:bg-slate-800 dark:text-slate-100',
	'Special category': 'bg-red-100 text-red-900 outline-red-300 dark:bg-red-950 dark:text-red-100'
};

export const tagColorClass = (tag: string) => TAG_COLOR_CLASSES[tag] ?? TAG_COLOR_CLASSES.Custom;

export const TAG_OPTIONS = ['Age', 'Gender', 'Ethnicity', 'Postcode', 'Custom'];
export const COUNTRY_OPTIONS = [
	'Scotland',
	'England',
	'Wales',
	'Northern Ireland',
	'United States'
];
/** Emoji flags shown next to the country name. Northern Ireland has no emoji flag, so it uses the UK flag. */
export const COUNTRY_FLAGS: Record<string, string> = {
	Scotland: '🏴󠁧󠁢󠁳󠁣󠁴󠁿',
	England: '🏴󠁧󠁢󠁥󠁮󠁧󠁿',
	Wales: '🏴󠁧󠁢󠁷󠁬󠁳󠁿',
	'Northern Ireland': '🇬🇧',
	'United States': '🇺🇸'
};
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
	maxSelections: number | null;
	placeholder: string;
	specialCategory: boolean;
	/** Wording of the consent checkbox shown with special category questions */
	consentText: string;
};

export type FormUsage = {
	conversationId: string;
	title: string;
	stage: 'draft' | 'live' | 'closed';
	pinnedVersion: number | null;
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
	usage: FormUsage[];
	hasUnpublishedChanges: boolean;
	isNewlyCreated: boolean;
	questions: FormQuestion[];
};

export const isChoiceKind = (kind: QuestionKind) =>
	kind === 'single_choice' || kind === 'multiple_choice';

const newId = (prefix: string) => `${prefix}-${Math.random().toString(36).slice(2, 8)}`;

export const DEFAULT_CONSENT_TEXT = 'I consent to the collection of this information about me.';

export const blankQuestion = (): DemographicQuestion => ({
	id: newId('q'),
	text: '',
	description: '',
	kind: 'single_choice',
	tags: ['Custom'],
	options: ['', '', ''],
	allowOther: false,
	preferNotToSay: true,
	maxSelections: null,
	placeholder: '',
	specialCategory: false,
	consentText: DEFAULT_CONSENT_TEXT
});

export const blankForm = (): DemographicForm => ({
	id: newId('f'),
	name: '',
	country: 'Scotland',
	status: 'draft',
	version: 0,
	createdBy: CURRENT_USER,
	editedLabel: 'Not yet published',
	usage: [],
	hasUnpublishedChanges: false,
	isNewlyCreated: false,
	questions: []
});

export const conversationCount = (form: DemographicForm) => form.usage.length;

export const versionInUse = (form: DemographicForm, usage: FormUsage) =>
	usage.pinnedVersion ?? form.version;

const followingLatest = (form: DemographicForm) =>
	form.usage.filter((u) => u.pinnedVersion === null);
const pinnedVersions = (form: DemographicForm) =>
	[
		...new Set(form.usage.flatMap((u) => (u.pinnedVersion === null ? [] : [u.pinnedVersion])))
	].sort((a, b) => b - a);

const plural = (count: number, one: string, many: string) => `${count} ${count === 1 ? one : many}`;

export const usageSummary = (form: DemographicForm) => {
	const parts: string[] = [];
	if (followingLatest(form).length > 0) {
		parts.push(`${followingLatest(form).length} follow latest`);
	}
	for (const version of pinnedVersions(form)) {
		const count = form.usage.filter((u) => u.pinnedVersion === version).length;
		parts.push(`${count} pinned to v${version}`);
	}
	return parts.join(', ');
};

export const publishImpact = (form: DemographicForm) => {
	if (conversationCount(form) === 0) {
		return 'No conversation uses this form yet, so nothing else changes.';
	}
	const parts: string[] = [];
	const following = followingLatest(form).length;
	if (following > 0) {
		parts.push(
			`${plural(following, 'conversation follows', 'conversations follow')} the latest version and will pick up this change.`
		);
	}
	const pinned = form.usage.length - following;
	if (pinned > 0) {
		const versions = pinnedVersions(form)
			.map((v) => `v${v}`)
			.join(', ');
		parts.push(
			`${plural(pinned, 'conversation is', 'conversations are')} pinned to ${versions} and will not change.`
		);
	}
	return parts.join(' ');
};

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
	maxSelections: null,
	placeholder: '',
	specialCategory: false,
	consentText: DEFAULT_CONSENT_TEXT,
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
		{
			allowOther: true,
			specialCategory: true,
			consentText: 'I consent to the collection of my ethnic group data.'
		}
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
		{
			kind: 'multiple_choice',
			specialCategory: true,
			consentText: 'I consent to the collection of data about my caring responsibilities.'
		}
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
		version: 2,
		createdBy: 'Shu',
		editedLabel: 'Edited 28 Sep by Andy',
		usage: [
			{
				conversationId: 'c-air',
				title: 'Enhance Air quality ...',
				stage: 'live',
				pinnedVersion: null
			},
			{
				conversationId: 'c-mock',
				title: 'Mocked up conversation...',
				stage: 'draft',
				pinnedVersion: null
			},
			{
				conversationId: 'c-ai',
				title: 'Scotland AI Playbook',
				stage: 'live',
				pinnedVersion: null
			},
			{
				conversationId: 'c-prs',
				title: 'Private Rented Sector',
				stage: 'live',
				pinnedVersion: 1
			}
		],
		hasUnpublishedChanges: false,
		isNewlyCreated: false,
		questions: formQuestions(['q-age', 'q-gender', 'q-birthday', 'q-postcode'], ['q-age'])
	},
	{
		id: 'f-social-prescribing',
		name: 'Social Prescribing team form',
		country: 'Scotland',
		status: 'published',
		version: 2,
		createdBy: 'Andy',
		editedLabel: 'Edited 2 Oct by Andy',
		usage: [
			{
				conversationId: 'c-sp',
				title: 'Social Prescribing pilot',
				stage: 'live',
				pinnedVersion: null
			}
		],
		hasUnpublishedChanges: true,
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
		usage: [],
		hasUnpublishedChanges: false,
		isNewlyCreated: false,
		questions: formQuestions(['q-age', 'q-gender', 'q-postcode', 'q-employment'])
	}
];
