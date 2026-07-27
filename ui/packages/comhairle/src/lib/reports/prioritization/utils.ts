import type { Question } from '@crownshy/api-client/api';

export function extractAxisDomain(question: Question): [number, number] | null {
	if (question.type.continuous) {
		return [question.type.continuous.min_value, question.type.continuous.max_value];
	}

	if (question.type.likert_scale) {
		const values = question.type.likert_scale.categories.map(
			(category) => category.value
		) as number[];

		return [Math.min(...values), Math.max(...values)];
	}

	return null;
}
