import type { Question } from '$lib/tools/prioritization';

export function extractAxisDomain(question: Question): [number, number] | null {
	if (question.type.kind === 'continuous') {
		return [question.type.minValue, question.type.maxValue];
	}

	if (question.type.kind === 'likert') {
		const values = question.type.categories.map((category) => category.value) as number[];

		return [Math.min(...values), Math.max(...values)];
	}

	return null;
}
