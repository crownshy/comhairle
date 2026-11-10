import type { DatePrecision, QuestionLatestDto, QuestionType2 } from '@crownshy/api-client/api';

export const DATE_PRECISION_LABELS: Record<DatePrecision, string> = {
	day: 'Day (YYYY/MM/DD)',
	month: 'Month (YYYY/MM)',
	year: 'Year (YYYY)'
};

export function getPlaceholder(kind: QuestionType2, datePrecision?: DatePrecision): string {
	switch (kind) {
		case 'single_select':
			return '';
		case 'multi_select':
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
		case 'any_string':
			return 'Type your answer';
		case 'location':
			return 'Enter your location';
		case 'custom':
		default:
			return 'Type your answer';
	}
}

export const SPECIAL_CATEGORY_TAG = 'Special category';
export type TagName =
	'Age' | 'Gender' | 'Ethnicity' | 'Postcode' | 'Custom' | typeof SPECIAL_CATEGORY_TAG;

export type Tag = { id: string; name: TagName };

export type FormQuestion = { questionId: string; required: boolean; text: string };

export type DemographicForm = {
	id: string | null;
	name: string;
	questions: FormQuestion[];
	archived: boolean;
	publishedVersion: number | null;
	draftVersion: number | null;
};

export type SavedForm = DemographicForm & { id: string };

/** The version number the next publish will create. */
export const nextVersionNumber = (item: {
	draftVersion: number | null;
	publishedVersion: number | null;
}) => item.draftVersion ?? (item.publishedVersion ?? 0) + 1;

export const formsUsingQuestion = (questionId: string, forms: SavedForm[]) =>
	forms.filter((form) => form.questions.some((q) => q.questionId === questionId));

export const createNewQuestion = (): QuestionLatestDto => ({
	id: '',
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
	publishedVersion: 0,
	draftVersion: 0
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
