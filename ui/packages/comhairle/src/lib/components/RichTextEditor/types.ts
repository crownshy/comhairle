export const CONTENT_TYPES = {
	JSON: 'json',
	MARKDOWN: 'markdown'
} as const;

export type ContentType = (typeof CONTENT_TYPES)[keyof typeof CONTENT_TYPES];

export type ActiveStates = {
	bold: boolean;
	italic: boolean;
	strike: boolean;
	code: boolean;
	underline: boolean;
	link: boolean;
	bulletList: boolean;
	orderedList: boolean;
	blockquote: boolean;
	heading: 'p' | '1' | '2' | '3';
	textAlign: 'left' | 'center' | 'right' | 'justify';
};
