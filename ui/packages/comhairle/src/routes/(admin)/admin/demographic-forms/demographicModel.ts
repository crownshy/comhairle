import type {
	DatePrecision,
	DemographicFormLatestDto,
	QuestionLatestDto
} from '@crownshy/api-client/api';

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

export function getPlaceholder(kind: QuestionKind, datePrecision: DatePrecision): string {
	switch (kind) {
		case 'single_choice':
			return '';
		case 'multiple_choice':
			return '';
		case 'number':
			return 'Enter a number';
		case 'date':
			switch (datePrecision) {
				case 'day':
					return 'YYYY/MM/DD';
				case 'month':
					return 'YYYY/MM';
				case 'year':
				default:
					return 'YYYY';
			}
		case 'open_text':
			return 'Type your answer';
		case 'postcode':
			return 'Enter your postcode';
		case 'lat_long':
			return 'Enter latitude and longitude';
		case 'country':
			return 'Enter your country';
		case 'custom':
		default:
			return 'Type your answer';
	}
}

export const isChoiceKind = (kind: QuestionKind) =>
	kind === 'single_choice' || kind === 'multiple_choice';

export const selectionHint = (kind: QuestionKind, maxSelections: number | null) => {
	if (kind !== 'multiple_choice') return '';
	if (maxSelections === null) return 'Select all that apply';
	return maxSelections === 1 ? 'Select one' : `Select up to ${maxSelections}`;
};

export const SPECIAL_CATEGORY_TAG = 'Special category';
export type TagName =
	'Age' | 'Gender' | 'Ethnicity' | 'Postcode' | 'Custom' | typeof SPECIAL_CATEGORY_TAG;

export type Tag = { id: string; name: TagName };

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

export const formsUsingQuestion = (questionId: string, forms: SavedForm[]) =>
	forms.filter((form) => form.questions.some((q) => q.questionId === questionId));

export const createNewQuestion = (): DemographicQuestion => ({
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
