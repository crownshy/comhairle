type ReportNode = {
	type?: string;
	attrs?: Record<string, unknown>;
	content?: ReportNode[];
	text?: string;
};

/** One page of the published report, and one pill in its header. */
export type ReportPage = {
	label: string;
	/** A TipTap document (or a legacy plain string) that `ReportBody` can render. */
	content: string;
};

function parseDocumentNodes(content: string): ReportNode[] | undefined {
	let document: unknown;
	try {
		document = JSON.parse(content);
	} catch {
		return undefined;
	}
	if (
		!document ||
		typeof document !== 'object' ||
		(document as ReportNode).type !== 'doc' ||
		!Array.isArray((document as ReportNode).content)
	) {
		return undefined;
	}
	return (document as { content: ReportNode[] }).content;
}

function nodeText(node: ReportNode): string {
	return (node.text ?? '') + (node.content ?? []).map(nodeText).join('');
}

function toDocument(nodes: ReportNode[]) {
	return JSON.stringify({ type: 'doc', content: nodes });
}

/** Nodes that show something without any text of their own. */
const CONTENT_WITHOUT_TEXT = new Set(['reportComponentEmbed', 'image', 'video', 'audio', 'iframe']);

/** True when a rich text value has any visible text, media or an embedded component. */
export function hasReportContent(content: string | null | undefined): boolean {
	if (!content) return false;
	const nodes = parseDocumentNodes(content);
	if (!nodes) return content.trim().length > 0;
	return nodes.some(
		(node) => CONTENT_WITHOUT_TEXT.has(node.type ?? '') || nodeText(node).trim().length > 0
	);
}

function isInline(node: ReportNode): boolean {
	return node.text !== undefined || node.type === 'hardBreak';
}

/** A node's text with a newline at each hard break and between blocks such as list items. */
function nodeLines(node: ReportNode): string {
	if (node.type === 'hardBreak') return '\n';
	if (node.text !== undefined) return node.text;
	const children = node.content ?? [];
	return children.map(nodeLines).join(children.some(isInline) ? '' : '\n');
}

/** The first line of text on a page, which names its pill. */
function firstLine(nodes: ReportNode[]): string | undefined {
	for (const node of nodes) {
		const line = nodeLines(node)
			.split('\n')
			.map((text) => text.trim())
			.find(Boolean);
		if (line) return line;
	}
	return undefined;
}

/**
 * Splits the report body into the pages the published report shows. Each page break (a
 * horizontal rule) starts a new page, and the page's first line of text names its pill.
 * Headings are only text styles. A page with nothing on it is dropped, and a page with no
 * text at all is named by `fallbackLabel`. See ADR-0069.
 */
export function splitReportIntoPages(
	content: string,
	fallbackLabel: (pageNumber: number) => string
): ReportPage[] {
	const nodes = parseDocumentNodes(content);
	if (!nodes) {
		return hasReportContent(content) ? [{ label: fallbackLabel(1), content }] : [];
	}

	const pagesNodes: ReportNode[][] = [[]];
	for (const node of nodes) {
		if (node.type === 'horizontalRule') pagesNodes.push([]);
		else pagesNodes[pagesNodes.length - 1].push(node);
	}

	return pagesNodes
		.filter((pageNodes) => hasReportContent(toDocument(pageNodes)))
		.map((pageNodes, index) => ({
			label: firstLine(pageNodes) ?? fallbackLabel(index + 1),
			content: toDocument(pageNodes)
		}));
}

const CARD_PARAM = 'card';

/**
 * Which screen the URL asks for: undefined for the report's first screen, 1 to `pageCount`
 * for a card, and `pageCount + 1` for the closing thank-you. Anything else falls back to the
 * first screen, so a stale link still opens the report.
 */
export function readCardNumber(url: URL, pageCount: number): number | undefined {
	const card = Number(url.searchParams.get(CARD_PARAM));
	return Number.isInteger(card) && card >= 1 && card <= pageCount + 1 ? card : undefined;
}

/** A link to a card, or to the report's first screen when `card` is left out. */
export function cardHref(url: URL, card?: number): string {
	const target = new URL(url);
	if (card === undefined) target.searchParams.delete(CARD_PARAM);
	else target.searchParams.set(CARD_PARAM, String(card));
	return target.pathname + target.search;
}
