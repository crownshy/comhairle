import { z } from 'zod';

const SLUG_PATTERN = /^[a-z0-9]+(?:[-_][a-z0-9]+)*$/;
const NOT_UUID_PATTERN = /^(?![0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}$)/;

export const detailsSchema = z.object({
	title: z
		.string()
		.min(1, 'Title is required')
		.max(200, 'Title must be less than 200 characters'),
	shortDescription: z
		.string()
		.min(1, 'Short description is required')
		.max(500, 'Short description must be less than 500 characters'),
	description: z.string().min(1, 'Description is required'),
	slug: z
		.string()
		.min(1, 'URL slug is required')
		.max(100, 'URL slug must be at most 100 characters')
		.regex(
			SLUG_PATTERN,
			'Use lowercase letters and numbers, separated by single hyphens or underscores'
		)
		.regex(NOT_UUID_PATTERN, "URL slug can't be a UUID")
});
