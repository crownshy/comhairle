export type ReportNode = {
	type: string;
	attrs?: Record<string, unknown>;
	content?: ReportNode[];
	text?: string;
	marks?: { type: string }[];
};

export type ReportDocument = { type: 'doc'; content: ReportNode[] };

/** A page pill in the editor: the top-level nodes between two page breaks (ADR-0069). */
export type ReportPage = {
	label: string;
	/** Nothing on it yet, so the published report leaves it out. */
	empty: boolean;
	/** Index of the node to scroll to: the page's first, or its page break while it is empty. */
	firstNodeIndex: number;
	/** The nodes to delete to remove the page, including the page break that separates it. */
	removal: { firstNodeIndex: number; nodeCount: number };
};

/**
 * The text the API fills a new report with (`api/src/models/report.rs`). The editor treats it as
 * unwritten, so a new report opens empty rather than on this sentence.
 */
const GENERATED_REPORT_TEXT = [
	'Summary to be filled out by facilitator',
	'Body to be filled out by facilitator'
];

export function parseReportDocument(content: string | null | undefined): ReportDocument | null {
	if (!content) return null;
	try {
		const parsed = JSON.parse(content);
		if (parsed?.type !== 'doc') return null;
		// TipTap stores an emptied document as `{"type":"doc"}`, with no content at all.
		return { type: 'doc', content: Array.isArray(parsed.content) ? parsed.content : [] };
	} catch {
		return null;
	}
}

function textOf(node: ReportNode): string {
	return (node.text ?? '') + (node.content ?? []).map(textOf).join('');
}

/** True when a summary or body has nothing the facilitator wrote: empty, blank, or the API default. */
export function isUnwritten(content: string | null | undefined): boolean {
	if (!content) return true;
	const document = parseReportDocument(content);
	if (!document) return GENERATED_REPORT_TEXT.includes(content.trim()) || !content.trim();
	// A chart, image, table or page break was put there on purpose, even with no text in it.
	const isTextBlock = (node: ReportNode) => node.type === 'paragraph' || node.type === 'heading';
	if (!document.content.every(isTextBlock)) return false;
	const text = document.content.map(textOf).join(' ').trim();
	return text === '' || GENERATED_REPORT_TEXT.includes(text);
}

/** Nodes that show something without any text of their own, as the published report counts them. */
const CONTENT_WITHOUT_TEXT = new Set(['reportComponentEmbed', 'image', 'video', 'audio', 'iframe']);

function hasContent(nodes: ReportNode[]): boolean {
	return nodes.some((node) => CONTENT_WITHOUT_TEXT.has(node.type) || textOf(node).trim() !== '');
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

function firstLine(nodes: ReportNode[]): string | undefined {
	for (const node of nodes) {
		const line = linesOf(node)
			.split('\n')
			.map((text) => text.trim())
			.find(Boolean);
		if (line) return line;
	}
	return undefined;
}

/**
 * The pages for a body, split the way the published report splits it: a page break starts a
 * page and the first line of text names it. Empty pages are listed too, called
 * `untitledLabel`, so a page shows up as soon as it is added; the published report drops them.
 */
export function reportPages(
	content: string | null | undefined,
	fallbackLabel: (pageNumber: number) => string,
	untitledLabel: string
): ReportPage[] {
	// A blank body is one empty page, as a new document is; legacy plain text has no pages.
	const document = content?.trim()
		? parseReportDocument(content)
		: { type: 'doc' as const, content: [] };
	if (!document) return [];

	const runs: { firstNodeIndex: number; nodes: ReportNode[] }[] = [
		{ firstNodeIndex: 0, nodes: [] }
	];
	document.content.forEach((node, index) => {
		if (node.type === 'horizontalRule') runs.push({ firstNodeIndex: index + 1, nodes: [] });
		else runs[runs.length - 1].nodes.push(node);
	});

	// "Page n" counts only pages with something on them, as the published report does.
	let publishedNumber = 0;
	return runs.map((run) => {
		const empty = !hasContent(run.nodes);
		if (!empty) publishedNumber += 1;
		// Take the page break before the page with it, or the one after for the opening page.
		const hasBreakBefore = run.firstNodeIndex > 0;
		const hasBreakAfter = run.firstNodeIndex + run.nodes.length < document.content.length;
		return {
			label: empty ? untitledLabel : (firstLine(run.nodes) ?? fallbackLabel(publishedNumber)),
			empty,
			firstNodeIndex:
				run.nodes.length === 0 && hasBreakBefore
					? run.firstNodeIndex - 1
					: run.firstNodeIndex,
			removal: {
				firstNodeIndex: run.firstNodeIndex - (hasBreakBefore ? 1 : 0),
				nodeCount: run.nodes.length + (hasBreakBefore || hasBreakAfter ? 1 : 0)
			}
		};
	});
}
