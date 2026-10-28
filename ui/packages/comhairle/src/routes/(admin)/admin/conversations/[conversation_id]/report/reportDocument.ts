import { sectionBlocksForTool } from '$lib/reports/embeds';
import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';

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
 * unwritten, so a new report opens on the starter draft rather than on this sentence.
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
	if (document.content.some((node) => node.type === 'reportComponentEmbed')) return false;
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

const PROMPTS = {
	introduction: 'Who took part, when, and what did you ask them?',
	turnout: 'How many people took part, and how did they join in?',
	consensus: 'What did people agree on?',
	anyChart: 'What does this show?',
	nextSteps: 'How will these findings be used, and when will people hear back?'
};

/** The charts a starter draft opens a step's section with. Other tools get their first block. */
const DRAFT_CHARTS_BY_TOOL: Record<string, { type: string; prompt: string }[]> = {
	polis: [
		{ type: 'polis-key-stats', prompt: PROMPTS.turnout },
		{ type: 'polis-area-consensus', prompt: PROMPTS.consensus }
	]
};

/** How many starter-draft prompts are still in a body, word for word, so publishing can warn. */
export function countDraftPrompts(content: string | null | undefined): number {
	const prompts = new Set(Object.values(PROMPTS));
	return (
		parseReportDocument(content)?.content.filter(
			(node) => node.type === 'paragraph' && prompts.has(textOf(node).trim())
		).length ?? 0
	);
}

function prompt(text: string): ReportNode {
	return { type: 'paragraph', content: [{ type: 'text', text, marks: [{ type: 'italic' }] }] };
}

function heading(text: string): ReportNode {
	return { type: 'heading', attrs: { level: 2 }, content: [{ type: 'text', text }] };
}

function chart(toolStepId: string, componentType: string): ReportNode {
	return { type: 'reportComponentEmbed', attrs: { toolStepId, componentType, config: {} } };
}

/**
 * A starter body for a report nobody has written yet: an introduction prompt, one section per
 * step that has charts (with its main ones embedded under a prompt), and a closing section.
 * Prompts are italic so they read as text to replace.
 */
export function draftReport(steps: EmbeddableStep[]): ReportDocument {
	const stepSections = steps.flatMap((step) => {
		const blocks = sectionBlocksForTool(step.toolType);
		if (blocks.length === 0) return [];
		const picks = DRAFT_CHARTS_BY_TOOL[step.toolType] ?? [
			{ type: blocks[0].type, prompt: PROMPTS.anyChart }
		];
		return [
			heading(step.name),
			...picks.flatMap((pick) => [prompt(pick.prompt), chart(step.id, pick.type)])
		];
	});
	return {
		type: 'doc',
		content: [
			prompt(PROMPTS.introduction),
			...stepSections,
			heading('What happens next'),
			prompt(PROMPTS.nextSteps)
		]
	};
}
