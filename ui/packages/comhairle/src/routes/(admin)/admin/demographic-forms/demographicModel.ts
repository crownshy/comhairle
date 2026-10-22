import type { DatePrecision } from '@crownshy/api-client/api';

export type QuestionKind =
	| 'single_choice'
	| 'multiple_choice'
	| 'number'
	| 'date'
	| 'open_text'
	| 'postcode'
	| 'lat_long'
	| 'country'
	| 'custom';

export const QUESTION_KIND_LABELS: Record<QuestionKind, string> = {
	single_choice: 'Single choice',
	multiple_choice: 'Multiple choice',
	number: 'Number',
	date: 'Date',
	open_text: 'Open text',
	postcode: 'Postcode, by region',
	lat_long: 'Latitude and longitude',
	country: 'Country',
	custom: 'Custom (set through the API)'
};

export const DATE_PRECISION_LABELS: Record<DatePrecision, string> = {
	day: 'Day (YYYY/MM/DD)',
	month: 'Month (YYYY/MM)',
	year: 'Year (YYYY)'
};

const DATE_PLACEHOLDERS: Record<DatePrecision, string> = {
	day: 'YYYY/MM/DD',
	month: 'YYYY/MM',
	year: 'YYYY'
};

const DEFAULT_PLACEHOLDERS: Record<Exclude<QuestionKind, 'date'>, string> = {
	single_choice: '',
	multiple_choice: '',
	number: 'Enter a number',
	open_text: 'Type your answer',
	postcode: 'Enter your postcode',
	lat_long: 'Enter latitude and longitude',
	country: 'Enter your country',
	custom: 'Type your answer'
};

export const isChoiceKind = (kind: QuestionKind) =>
	kind === 'single_choice' || kind === 'multiple_choice';

export const selectionHint = (kind: QuestionKind, maxSelections: number | null) => {
	if (kind !== 'multiple_choice') return '';
	if (maxSelections === null) return 'Select all that apply';
	return maxSelections === 1 ? 'Select one' : `Select up to ${maxSelections}`;
};

export const defaultPlaceholder = (kind: QuestionKind, datePrecision: DatePrecision) =>
	kind === 'date' ? DATE_PLACEHOLDERS[datePrecision] : DEFAULT_PLACEHOLDERS[kind];

const TAG_COLOR_CLASSES: Record<string, string> = {
	Age: 'bg-blue-100 text-blue-900 outline-blue-300 dark:bg-blue-950 dark:text-blue-100',
	Gender: 'bg-purple-100 text-purple-900 outline-purple-300 dark:bg-purple-950 dark:text-purple-100',
	Ethnicity:
		'bg-amber-100 text-amber-900 outline-amber-300 dark:bg-amber-950 dark:text-amber-100',
	Postcode: 'bg-green-100 text-green-900 outline-green-300 dark:bg-green-950 dark:text-green-100',
	Custom: 'bg-slate-100 text-slate-800 outline-slate-300 dark:bg-slate-800 dark:text-slate-100',
	'Special category': 'bg-red-100 text-red-900 outline-red-300 dark:bg-red-950 dark:text-red-100'
};

export const SPECIAL_CATEGORY_TAG = 'Special category';

export const tagColorClass = (tag: string) => TAG_COLOR_CLASSES[tag] ?? TAG_COLOR_CLASSES.Custom;

export type Tag = { id: string; name: string };

export type DemographicQuestion = {
	id: string | null;
	isDefault: boolean;
	text: string;
	kind: QuestionKind;
	datePrecision: DatePrecision;
	locationCountry: string | null;
	tags: Tag[];
	options: string[];
	allowOther: boolean;
	preferNotToSay: boolean;
	maxSelections: number | null;
	specialCategory: boolean;
	consentText: string;
	archived: boolean;
	publishedVersion: number | null;
	draftVersion: number | null;
};

export type FormQuestion = { questionId: string; required: boolean; text: string };

export type DemographicForm = {
	id: string | null;
	name: string;
	questions: FormQuestion[];
	archived: boolean;
	publishedVersion: number | null;
	draftVersion: number | null;
};

export type SavedQuestion = DemographicQuestion & { id: string };
export type SavedForm = DemographicForm & { id: string };

/** The version number the next publish will create. */
export const nextVersionNumber = (item: {
	draftVersion: number | null;
	publishedVersion: number | null;
}) => item.draftVersion ?? (item.publishedVersion ?? 0) + 1;

/** Short human reference shown alongside a question or form, derived from its id. */
export const shortId = (prefix: 'F' | 'Q', id: string | null) =>
	id ? `${prefix}-${id.slice(0, 8).toUpperCase()}` : `${prefix}-NEW`;

export const formsUsingQuestion = (questionId: string, forms: SavedForm[]) =>
	forms.filter((form) => form.questions.some((q) => q.questionId === questionId));

export const blankQuestion = (): DemographicQuestion => ({
	id: null,
	isDefault: false,
	text: '',
	kind: 'single_choice',
	datePrecision: 'day',
	locationCountry: null,
	tags: [],
	options: ['', '', ''],
	allowOther: false,
	preferNotToSay: true,
	maxSelections: null,
	specialCategory: false,
	consentText: '',
	archived: false,
	publishedVersion: null,
	draftVersion: null
});

export const duplicateQuestionDraft = (question: DemographicQuestion): DemographicQuestion => ({
	...question,
	id: null,
	isDefault: false,
	text: `${question.text} (copy)`,
	tags: question.tags.map((tag) => ({ ...tag })),
	options: [...question.options],
	archived: false,
	publishedVersion: null,
	draftVersion: null
});

export const blankForm = (): DemographicForm => ({
	id: null,
	name: '',
	questions: [],
	archived: false,
	publishedVersion: null,
	draftVersion: null
});

export const specialCategoryCount = (form: DemographicForm, questions: DemographicQuestion[]) =>
	form.questions.filter((fq) => questions.find((q) => q.id === fq.questionId)?.specialCategory)
		.length;
