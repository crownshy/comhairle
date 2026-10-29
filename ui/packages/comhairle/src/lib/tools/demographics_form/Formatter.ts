import type { QuestionType2 } from '@crownshy/api-client/api';

function from(type: QuestionType2, value: string[]) {
	switch (type) {
		case 'number':
			return Number(value[0]);
		case 'date':
			return value[0].replaceAll('-', '/');
		case 'single_select':
		case 'multi_select':
			return { option_ids: value };
		case 'location':
		case 'any_string':
		case 'custom':
		default:
			return value[0];
	}
}

function to(type: QuestionType2, value: unknown): unknown {
	switch (type) {
		case 'date':
			return value?.replaceAll('/', '-');
		case 'single_select':
		case 'multi_select':
			return value?.option_ids ?? [];
		case 'number':
		case 'location':
		case 'any_string':
		case 'custom':
		default:
			return value;
	}
}

export default {
	to,
	from
};
