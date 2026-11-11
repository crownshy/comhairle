import type { QuestionType } from '@crownshy/api-client/api';

function from(type: QuestionType, value: string[]) {
	switch (type) {
		case 'number':
			return Number(value[0]);
		case 'date':
			return value[0].replaceAll('-', '/');
		case 'select':
			return { option_ids: value };
		case 'location':
		case 'text':
		case 'custom':
		default:
			return value[0];
	}
}

function to(type: QuestionType, value: unknown): unknown {
	switch (type) {
		case 'date':
			return value?.replaceAll('/', '-');
		case 'select':
			return value?.option_ids ?? [];
		case 'number':
		case 'location':
		case 'text':
		case 'custom':
		default:
			return value;
	}
}

export default {
	to,
	from
};
