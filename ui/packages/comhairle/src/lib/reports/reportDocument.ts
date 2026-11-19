/** A node in a report's TipTap document, as stored in the summary or body. */
export type ReportNode = {
	type?: string;
	attrs?: Record<string, unknown>;
	content?: ReportNode[];
	text?: string;
	marks?: { type: string }[];
};

/**
 * The text the API fills a new report with (`api/src/models/report.rs`). Nobody wrote it, so the
 * editor opens it as empty and the published report leaves it out.
 */
const GENERATED_REPORT_TEXT = [
	'Summary to be filled out by facilitator',
	'Body to be filled out by facilitator'
];

export function isGeneratedReportText(text: string): boolean {
	return GENERATED_REPORT_TEXT.includes(text.trim());
}

/** The top-level nodes of a stored document, or undefined for legacy plain text. */
export function parseDocumentNodes(content: string | null | undefined): ReportNode[] | undefined {
	if (!content) return undefined;
	let parsed: unknown;
	try {
		parsed = JSON.parse(content);
	} catch {
		return undefined;
	}
	if (!parsed || typeof parsed !== 'object' || (parsed as ReportNode).type !== 'doc') {
		return undefined;
	}
	// TipTap stores an emptied document as `{"type":"doc"}`, with no content at all.
	const nodes = (parsed as ReportNode).content;
	return Array.isArray(nodes) ? nodes : [];
}

export function textOf(node: ReportNode): string {
	return (node.text ?? '') + (node.content ?? []).map(textOf).join('');
}

/** Nodes that show something without any text of their own. */
const CONTENT_WITHOUT_TEXT = new Set(['reportComponentEmbed', 'image', 'video', 'audio', 'iframe']);

/** True when the nodes show any text, media or embedded component, other than the API's default. */
export function nodesHaveContent(nodes: ReportNode[]): boolean {
	if (nodes.some((node) => CONTENT_WITHOUT_TEXT.has(node.type ?? ''))) return true;
	const text = nodes.map(textOf).join(' ').trim();
	return text !== '' && !isGeneratedReportText(text);
}

/** True when a summary or body has something a reader should see. */
export function hasReportContent(content: string | null | undefined): boolean {
	if (!content) return false;
	const nodes = parseDocumentNodes(content);
	if (!nodes) return content.trim() !== '' && !isGeneratedReportText(content);
	return nodesHaveContent(nodes);
}

function isInline(node: ReportNode): boolean {
	return node.text !== undefined || node.type === 'hardBreak';
}

/** A node's text with a newline at each hard break and between blocks such as list items. */
function linesOf(node: ReportNode): string {
	if (node.type === 'hardBreak') return '\n';
	if (node.text !== undefined) return node.text;
	const children = node.content ?? [];
	return children.map(linesOf).join(children.some(isInline) ? '' : '\n');
}

/** The first line of text in the nodes, which names a page (ADR-0069). */
export function firstLine(nodes: ReportNode[]): string | undefined {
	for (const node of nodes) {
		const line = linesOf(node)
			.split('\n')
			.map((text) => text.trim())
			.find(Boolean);
		if (line) return line;
	}
	return undefined;
}
