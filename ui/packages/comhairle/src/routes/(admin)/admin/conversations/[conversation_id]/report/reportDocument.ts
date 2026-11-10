export type ReportNode = {
	type: string;
	attrs?: Record<string, unknown>;
	content?: ReportNode[];
	text?: string;
	marks?: { type: string }[];
};

export type ReportDocument = { type: 'doc'; content: ReportNode[] };

/** A section pill in the editor: a run of top-level nodes headed by an H2 (ADR-0058). */
export type ReportSection = {
	label: string;
	/** Position among all sections, including an untitled introduction. */
	index: number;
	/** Index of the section's first top-level node, for scrolling to it. */
	firstNodeIndex: number;
	/** How many top-level nodes the section spans, for removing it. */
	nodeCount: number;
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

function isSectionHeading(node: ReportNode): boolean {
	return node.type === 'heading' && node.attrs?.level === 2;
}

function splitIntoSections(
	document: ReportDocument
): { title: string | null; nodes: ReportNode[] }[] {
	const sections: { title: string | null; nodes: ReportNode[] }[] = [{ title: null, nodes: [] }];
	for (const node of document.content) {
		if (isSectionHeading(node)) sections.push({ title: textOf(node), nodes: [node] });
		else sections[sections.length - 1].nodes.push(node);
	}
	return sections;
}

/** The section pills for a body, matching how the published report splits it. */
export function reportSections(
	content: string | null | undefined,
	introductionLabel: string
): ReportSection[] {
	const document = parseReportDocument(content);
	if (!document) return [];
	let firstNodeIndex = 0;
	return splitIntoSections(document).flatMap((section, index) => {
		const start = firstNodeIndex;
		firstNodeIndex += section.nodes.length;
		if (section.nodes.length === 0) return [];
		return [
			{
				label:
					section.title?.trim() ||
					(section.title === null ? introductionLabel : 'Untitled'),
				index,
				firstNodeIndex: start,
				nodeCount: section.nodes.length
			}
		];
	});
}
