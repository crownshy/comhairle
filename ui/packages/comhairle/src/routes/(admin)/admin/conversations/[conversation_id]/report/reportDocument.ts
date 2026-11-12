import {
	firstLine,
	isGeneratedReportText,
	nodesHaveContent,
	parseDocumentNodes,
	textOf,
	type ReportNode
} from '$lib/reports/reportDocument';

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

/** True when a summary or body has nothing the facilitator wrote: empty, blank, or the API default. */
export function isUnwritten(content: string | null | undefined): boolean {
	if (!content) return true;
	const nodes = parseDocumentNodes(content);
	if (!nodes) return !content.trim() || isGeneratedReportText(content);
	// A chart, image, table or page break was put there on purpose, even with no text in it.
	const isTextBlock = (node: ReportNode) => node.type === 'paragraph' || node.type === 'heading';
	if (!nodes.every(isTextBlock)) return false;
	const text = nodes.map(textOf).join(' ').trim();
	return text === '' || isGeneratedReportText(text);
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
	const nodes = content?.trim() ? parseDocumentNodes(content) : [];
	if (!nodes) return [];

	const runs: { firstNodeIndex: number; nodes: ReportNode[] }[] = [
		{ firstNodeIndex: 0, nodes: [] }
	];
	nodes.forEach((node, index) => {
		if (node.type === 'horizontalRule') runs.push({ firstNodeIndex: index + 1, nodes: [] });
		else runs[runs.length - 1].nodes.push(node);
	});

	// "Page n" counts only pages with something on them, as the published report does.
	let publishedNumber = 0;
	return runs.map((run) => {
		const empty = !nodesHaveContent(run.nodes);
		if (!empty) publishedNumber += 1;
		// Take the page break before the page with it, or the one after for the opening page.
		const hasBreakBefore = run.firstNodeIndex > 0;
		const hasBreakAfter = run.firstNodeIndex + run.nodes.length < nodes.length;
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
