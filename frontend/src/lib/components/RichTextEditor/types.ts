export const CONTENT_TYPES = {
	JSON: 'json',
	MARKDOWN: 'markdown'
} as const;

export type ContentType = typeof CONTENT_TYPES[keyof typeof CONTENT_TYPES];
