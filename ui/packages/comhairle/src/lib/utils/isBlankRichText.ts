import { extractTextFromTiptap, isTiptapJson } from './tiptapUtils';

/** A cleared rich editor still saves markup like `<p></p>`, or a TipTap doc with one empty paragraph. */
export function isBlankRichText(content: string | null | undefined): boolean {
	const raw = content ?? '';
	const text = isTiptapJson(raw)
		? extractTextFromTiptap(raw)
		: raw.replace(/<[^>]*>/g, '').replace(/&nbsp;/g, '');
	return text.trim().length === 0;
}
