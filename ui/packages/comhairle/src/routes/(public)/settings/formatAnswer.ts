import type { QuestionVersionDto } from '@crownshy/api-client/api';

type Version = Pick<QuestionVersionDto, 'question_type' | 'options'>;

const isRecord = (value: unknown): value is Record<string, unknown> =>
	typeof value === 'object' && value !== null && !Array.isArray(value);

function formatSelection(version: Version, value: Record<string, unknown>) {
	const labels = new Map(version.options.map((option) => [option.id, option.label]));
	const optionIds = Array.isArray(value.option_ids) ? value.option_ids : [];
	const parts = optionIds.map((id) => labels.get(String(id)) ?? String(id));
	if (typeof value.other_text === 'string' && value.other_text.trim() !== '') {
		parts.push(`Other: ${value.other_text.trim()}`);
	}
	if (value.prefer_not_to_say === true) parts.push('Prefer not to say');
	return parts.join(', ');
}

function formatLocation(value: Record<string, unknown>) {
	if (typeof value.postcode === 'string') return value.postcode;
	if (typeof value.country === 'string') return value.country;
	if (value.latitude !== undefined && value.longitude !== undefined) {
		return `${value.latitude}, ${value.longitude}`;
	}
	return JSON.stringify(value);
}

/** A readable, one-line version of a saved demographic answer. */
export function formatAnswer(version: Version, value: unknown): string {
	if (typeof value === 'string' || typeof value === 'number') return String(value);
	if (!isRecord(value)) return JSON.stringify(value);

	switch (version.question_type) {
		case 'single_select':
		case 'multi_select':
			return formatSelection(version, value);
		case 'location':
			return formatLocation(value);
		default:
			return JSON.stringify(value);
	}
}
