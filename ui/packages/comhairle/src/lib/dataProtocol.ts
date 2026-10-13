import { m } from '$lib/paraglide/messages';
import { toolMeta } from '$lib/tool_meta';
import { isBlankRichText } from '$lib/utils/isBlankRichText';

/** The text participants see when the admin leaves the step's Data protocol blank. */
export function defaultDataProtocol(toolType: string | undefined): string {
	const text = toolMeta(toolType)?.dataProtocolDefault ?? m.data_protocol_default_generic;
	return text();
}

/** Defaults separate paragraphs with a blank line, since messages are plain text. */
export function defaultDataProtocolParagraphs(toolType: string | undefined): string[] {
	return defaultDataProtocol(toolType).split(/\n\s*\n/);
}

function escapeHtml(text: string): string {
	return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

/** Default text as rich-editor HTML, for an admin who wants to start from it. */
export function defaultDataProtocolHtml(toolType: string | undefined): string {
	return defaultDataProtocolParagraphs(toolType)
		.map((paragraph) => `<p>${escapeHtml(paragraph)}</p>`)
		.join('');
}

export function toolUsesAI(toolType: string | undefined): boolean {
	return toolMeta(toolType)?.usesAI ?? false;
}

export function hasDataProtocolText(html: string | null | undefined): boolean {
	return !isBlankRichText(html);
}
