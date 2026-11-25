import { m } from '$lib/paraglide/messages';
import { toolMeta } from '$lib/tool_meta';
import { isBlankRichText } from '$lib/utils/isBlankRichText';
import { extractTextFromTiptap, isTiptapJson, wrapTextInTiptap } from '$lib/utils/tiptapUtils';

/** The text participants see when the admin leaves the step's Data protocol blank. */
export function defaultDataProtocol(toolType: string | undefined): string {
	const text = toolMeta(toolType)?.dataProtocolDefault ?? m.data_protocol_default_generic;
	return text();
}

/** Defaults separate paragraphs with a blank line, since messages are plain text. */
export function defaultDataProtocolParagraphs(toolType: string | undefined): string[] {
	return defaultDataProtocol(toolType).split(/\n\s*\n/);
}

/** Default text as a TipTap doc, so the admin editor can start from it. */
export function defaultDataProtocolDoc(toolType: string | undefined): string {
	return wrapTextInTiptap(defaultDataProtocolParagraphs(toolType).join('\n\n'));
}

// Paragraph breaks and markup differ between TipTap JSON and older HTML saves, so compare words.
function comparableText(content: string): string {
	const text = isTiptapJson(content)
		? extractTextFromTiptap(content)
		: content.replace(/<[^>]*>/g, ' ').replace(/&nbsp;/g, ' ');
	return text.replace(/[\s-]+/g, ' ').trim();
}

/** Blank, or the tool's default word for word, so it can stay the shared, translated default. */
export function isDefaultDataProtocol(
	content: string | null | undefined,
	toolType: string | undefined
): boolean {
	if (!hasDataProtocolText(content)) return true;
	return comparableText(content ?? '') === comparableText(defaultDataProtocolDoc(toolType));
}

export function toolUsesAI(toolType: string | undefined): boolean {
	return toolMeta(toolType)?.usesAI ?? false;
}

export function hasDataProtocolText(html: string | null | undefined): boolean {
	return !isBlankRichText(html);
}
