import type { QuestionKind } from './demographicPrototypeData';

export const PREFER_NOT_TO_SAY = 'Prefer not to say';
export const OTHER_CHOICE = 'Other (please specify)';

export type Answer = { selected: string[]; otherText: string; text: string };

export const emptyAnswer = (): Answer => ({ selected: [], otherText: '', text: '' });

const countableSelections = (answer: Answer) =>
	answer.selected.filter((choice) => choice !== PREFER_NOT_TO_SAY).length;

export function isChoiceDisabled(
	answer: Answer,
	choice: string,
	kind: QuestionKind,
	maxSelections: number | null
) {
	if (kind !== 'multiple_choice' || maxSelections === null) return false;
	if (choice === PREFER_NOT_TO_SAY || answer.selected.includes(choice)) return false;
	return countableSelections(answer) >= maxSelections;
}

export function toggleChoice(
	answer: Answer,
	choice: string,
	kind: QuestionKind,
	maxSelections: number | null
): Answer {
	let selected: string[];
	if (answer.selected.includes(choice)) {
		selected = answer.selected.filter((value) => value !== choice);
	} else if (kind === 'single_choice' || choice === PREFER_NOT_TO_SAY) {
		selected = [choice];
	} else if (isChoiceDisabled(answer, choice, kind, maxSelections)) {
		return answer;
	} else {
		selected = [...answer.selected.filter((value) => value !== PREFER_NOT_TO_SAY), choice];
	}
	return {
		...answer,
		selected,
		otherText: selected.includes(OTHER_CHOICE) ? answer.otherText : ''
	};
}
