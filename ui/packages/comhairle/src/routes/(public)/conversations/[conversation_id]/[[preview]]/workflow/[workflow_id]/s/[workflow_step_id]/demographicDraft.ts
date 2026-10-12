import type { DatePrecision, PendingQuestion, QuestionVersionDto } from '@crownshy/api-client/api';

export type Draft = {
	text: string;
	latitude: string;
	longitude: string;
	optionIds: string[];
	otherSelected: boolean;
	otherText: string;
	preferNotToSay: boolean;
};

export type AnswerInput =
	| 'number'
	| 'date'
	| 'text'
	| 'postcode'
	| 'lat_long'
	| 'country'
	| 'single'
	| 'multiple'
	| 'custom';

export type ChoiceRules = { multiple: boolean; maxSelections: number | null };

export const OTHER_CHOICE = 'other';
export const PREFER_NOT_TO_SAY_CHOICE = 'prefer_not_to_say';

export const DATE_FORMATS: Record<DatePrecision, string> = {
	day: 'YYYY/MM/DD',
	month: 'YYYY/MM',
	year: 'YYYY'
};

const DATE_PATTERNS: Record<DatePrecision, RegExp> = {
	day: /^\d{4}\/\d{2}\/\d{2}$/,
	month: /^\d{4}\/\d{2}$/,
	year: /^\d{4}$/
};

export const emptyDraft = (): Draft => ({
	text: '',
	latitude: '',
	longitude: '',
	optionIds: [],
	otherSelected: false,
	otherText: '',
	preferNotToSay: false
});

export function answerInput(version: QuestionVersionDto): AnswerInput {
	switch (version.question_type) {
		case 'number':
			return 'number';
		case 'date':
			return 'date';
		case 'any_string':
			return 'text';
		case 'location':
			if (version.location_format === 'lat_long') return 'lat_long';
			if (version.location_format === 'country') return 'country';
			return 'postcode';
		case 'single_select':
			return 'single';
		case 'multi_select':
			return 'multiple';
		case 'custom':
			return 'custom';
	}
}

export function choiceRules(version: QuestionVersionDto): ChoiceRules {
	return {
		multiple: version.question_type === 'multi_select',
		maxSelections: version.max_selections ?? null
	};
}

export function needsConsent(question: PendingQuestion): boolean {
	return question.version.special_category && question.consent === 'none';
}

export function hasContent(draft: Draft): boolean {
	return (
		[draft.text, draft.latitude, draft.longitude].some((value) => value.trim() !== '') ||
		draft.optionIds.length > 0 ||
		draft.otherSelected ||
		draft.preferNotToSay
	);
}

/** The API value for the draft, or undefined when the draft is not a valid answer yet. */
export function buildAnswer(version: QuestionVersionDto, draft: Draft): unknown {
	switch (answerInput(version)) {
		case 'number':
			return numberValue(draft.text);
		case 'date':
			return dateValue(version.date_precision ?? 'day', draft.text);
		case 'text':
			return trimmedOrUndefined(draft.text);
		case 'postcode': {
			const postcode = trimmedOrUndefined(draft.text);
			return postcode === undefined ? undefined : { postcode };
		}
		case 'country': {
			const country = trimmedOrUndefined(draft.text);
			return country === undefined ? undefined : { country };
		}
		case 'lat_long':
			return latLongValue(draft.latitude, draft.longitude);
		case 'single':
		case 'multiple':
			return selectionValue(choiceRules(version), draft);
		case 'custom':
			return customValue(draft.text);
	}
}

/** Whether the question no longer blocks the step: answered, reused, declined, or optional and left blank. */
export function isQuestionSettled(question: PendingQuestion, draft: Draft): boolean {
	if (question.skipped || question.answer) return true;
	if (needsConsent(question)) return !question.required;
	if (buildAnswer(question.version, draft) !== undefined) return true;
	return !question.required && !hasContent(draft);
}

export function selectedChoice(draft: Draft): string {
	if (draft.preferNotToSay) return PREFER_NOT_TO_SAY_CHOICE;
	if (draft.otherSelected) return OTHER_CHOICE;
	return draft.optionIds[0] ?? '';
}

export function isChoiceDisabled(draft: Draft, choice: string, rules: ChoiceRules): boolean {
	if (!rules.multiple || rules.maxSelections === null) return false;
	if (choice === OTHER_CHOICE || choice === PREFER_NOT_TO_SAY_CHOICE) return false;
	return !draft.optionIds.includes(choice) && draft.optionIds.length >= rules.maxSelections;
}

/** Other and prefer-not-to-say are exclusive with options, and prefer-not-to-say is exclusive with everything. */
export function setChoice(
	draft: Draft,
	choice: string,
	selected: boolean,
	rules: ChoiceRules
): Draft {
	if (choice === PREFER_NOT_TO_SAY_CHOICE) {
		return selected
			? { ...emptyDraft(), preferNotToSay: true }
			: { ...draft, preferNotToSay: false };
	}
	if (!rules.multiple) {
		return choice === OTHER_CHOICE
			? { ...emptyDraft(), otherSelected: true }
			: { ...emptyDraft(), optionIds: [choice] };
	}
	const cleared = { ...draft, preferNotToSay: false };
	if (choice === OTHER_CHOICE) {
		return { ...cleared, otherSelected: selected, otherText: selected ? draft.otherText : '' };
	}
	const remaining = draft.optionIds.filter((id) => id !== choice);
	if (!selected) return { ...cleared, optionIds: remaining };
	if (isChoiceDisabled(draft, choice, rules)) return draft;
	return { ...cleared, optionIds: [...remaining, choice] };
}

function trimmedOrUndefined(text: string): string | undefined {
	const trimmed = text.trim();
	return trimmed === '' ? undefined : trimmed;
}

function numberValue(text: string): number | undefined {
	if (text.trim() === '') return undefined;
	const value = Number(text);
	return Number.isFinite(value) ? value : undefined;
}

function dateValue(precision: DatePrecision, text: string): string | undefined {
	const value = text.trim();
	if (!DATE_PATTERNS[precision].test(value)) return undefined;
	const [year, month = 1, day = 1] = value.split('/').map(Number);
	return isCalendarDate(year, month, day) ? value : undefined;
}

function isCalendarDate(year: number, month: number, day: number): boolean {
	const leap = (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
	const daysInMonth = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
	return month >= 1 && month <= 12 && day >= 1 && day <= daysInMonth[month - 1];
}

function latLongValue(latitude: string, longitude: string) {
	if (latitude.trim() === '' || longitude.trim() === '') return undefined;
	const lat = Number(latitude);
	const long = Number(longitude);
	const valid =
		Number.isFinite(lat) &&
		Number.isFinite(long) &&
		lat >= -90 &&
		lat <= 90 &&
		long >= -180 &&
		long <= 180;
	return valid ? { latitude: lat, longitude: long } : undefined;
}

function selectionValue(rules: ChoiceRules, draft: Draft) {
	if (draft.preferNotToSay) return { option_ids: [], prefer_not_to_say: true };
	if (rules.maxSelections !== null && draft.optionIds.length > rules.maxSelections) {
		return undefined;
	}
	const otherText = draft.otherSelected ? draft.otherText.trim() : '';
	if (draft.otherSelected && otherText === '') return undefined;
	if (draft.optionIds.length === 0 && !draft.otherSelected) return undefined;
	return draft.otherSelected
		? { option_ids: draft.optionIds, other_text: otherText }
		: { option_ids: draft.optionIds };
}

function customValue(text: string): unknown {
	try {
		return JSON.parse(text);
	} catch {
		return undefined;
	}
}
