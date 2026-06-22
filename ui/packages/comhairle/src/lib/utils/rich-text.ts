import { getBaseExtensions } from '$lib/components/RichTextEditor/editorConfig';
import { generateHTML } from '@tiptap/core';

export function jsonToHtml(jsonString: string): string {
	try {
		const json = JSON.parse(jsonString);
		return generateHTML(json, getBaseExtensions({ mode: 'renderer' }));
	} catch (e) {
		console.error(e);
		return '';
	}
}
