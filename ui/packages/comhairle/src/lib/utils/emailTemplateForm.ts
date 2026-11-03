import type { EmailTypeSchema } from '@crownshy/api-client/api';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
import { detectContentType } from './contentDetection';
import { jsonToHtml } from './rich-text';

/** Slot values keyed by slot key, as the form holds them. */
export type SlotValues = Record<string, string>;

/**
 * Turns one rich text value into the HTML the API stores. The editor only reports edits,
 * so a slot nobody touched still holds the HTML it was loaded with and must pass through
 * unchanged rather than be parsed as editor JSON (which would empty it).
 */
export function richTextToHtml(value: string): string {
	return detectContentType(value).type === CONTENT_TYPES.JSON ? jsonToHtml(value) : value;
}

/**
 * The slot payload for create, update and preview: every slot the schema defines, rich
 * text converted to HTML.
 */
export function buildSlotPayload(schema: EmailTypeSchema, values: SlotValues): SlotValues {
	const payload: SlotValues = {};
	for (const slot of schema.slots) {
		const value = values[slot.key] ?? '';
		payload[slot.key] = slot.content_type === 'rich_text' ? richTextToHtml(value) : value;
	}
	return payload;
}

/** The starting slot values: a saved version's content, or the schema's defaults. */
export function initialSlotValues(
	schema: EmailTypeSchema,
	saved?: Record<string, unknown>
): SlotValues {
	const values: SlotValues = {};
	for (const slot of schema.slots) {
		const savedValue = saved?.[slot.key];
		values[slot.key] = typeof savedValue === 'string' ? savedValue : slot.default_content;
	}
	return values;
}

/** What the customise form hands to its page when saved. */
export type EmailTemplateSubmission = { subject: string; slots: SlotValues };

const VARIABLE_PATTERN = /\{\{\s*([a-z0-9_]+)\s*\}\}/gi;
const MARKER_PATTERN = /\[\[variable:([a-z0-9_]+)\]\]/gi;
const VARIABLE_HIGHLIGHT_STYLE =
	'background:#fff3bf;color:inherit;padding:0 2px;border-radius:2px;font-family:monospace';

/**
 * Swaps each `{{variable}}` for a marker the preview endpoint leaves alone, so the
 * preview can show the placeholders instead of the endpoint's example values.
 * Pair with {@link revealVariables} on the returned HTML.
 */
export function markVariables(slots: SlotValues): SlotValues {
	const marked: SlotValues = {};
	for (const [slotKey, value] of Object.entries(slots)) {
		marked[slotKey] = value.replace(VARIABLE_PATTERN, '[[variable:$1]]');
	}
	return marked;
}

/** Turns the markers from {@link markVariables} back into highlighted placeholders. */
export function revealVariables(html: string): string {
	return html.replace(MARKER_PATTERN, `<mark style="${VARIABLE_HIGHLIGHT_STYLE}">{{$1}}</mark>`);
}

/** Where the customise form is in saving, shown beside the header's save button. */
export type SaveStatus = 'idle' | 'unsaved' | 'saving' | 'saved' | 'error';

/** Variable values the admin page already knows for real, such as the chosen conversation. */
export type KnownVariables = Record<string, string>;

const HTML_ESCAPES: Record<string, string> = {
	'&': '&amp;',
	'<': '&lt;',
	'>': '&gt;',
	'"': '&quot;',
	"'": '&#39;'
};

function escapeHtml(text: string): string {
	return text.replace(/[&<>"']/g, (character) => HTML_ESCAPES[character]);
}

/**
 * Fills in the variables we know, leaving the rest for the preview endpoint's examples.
 * @param escape Escape the value for HTML, for rich text.
 */
export function fillKnownVariables(text: string, known: KnownVariables, escape = false): string {
	return text.replace(VARIABLE_PATTERN, (placeholder, name: string) => {
		if (!(name in known)) return placeholder;
		return escape ? escapeHtml(known[name]) : known[name];
	});
}

/** {@link fillKnownVariables} across a slot payload, escaping only rich text slots. */
export function fillSlotVariables(
	schema: EmailTypeSchema,
	slots: SlotValues,
	known: KnownVariables
): SlotValues {
	const filled: SlotValues = {};
	for (const slot of schema.slots) {
		const value = slots[slot.key] ?? '';
		filled[slot.key] = fillKnownVariables(value, known, slot.content_type === 'rich_text');
	}
	return filled;
}

// The mailer inlines its CSS, so the tag usually carries a style attribute too.
/** The chosen conversation, shown in the preview in place of the endpoint's examples. */
export type PreviewConversation = { title: string; imageUrl?: string | null };

const PREVIEW_TITLE = /(<h1[^>]*data-preview="conversation-title"[^>]*>)[\s\S]*?(<\/h1>)/;
const PREVIEW_IMAGE = /(<img[^>]*data-preview="conversation-image"[^>]*src=")[^"]*(")/;

/**
 * Swaps the invite's example title and banner for the chosen conversation's, using the
 * data-preview hooks in conversation_invite.html. Emails without the hooks pass through.
 */
export function applyPreviewConversation(
	html: string,
	conversation: PreviewConversation | null
): string {
	if (!conversation) return html;

	let result = html.replace(
		PREVIEW_TITLE,
		(_match, opening: string, closing: string) =>
			`${opening}${escapeHtml(conversation.title)}${closing}`
	);
	if (conversation.imageUrl?.startsWith('https://')) {
		const imageUrl = escapeHtml(conversation.imageUrl);
		result = result.replace(
			PREVIEW_IMAGE,
			(_match, before: string, after: string) => `${before}${imageUrl}${after}`
		);
	}
	return result;
}
