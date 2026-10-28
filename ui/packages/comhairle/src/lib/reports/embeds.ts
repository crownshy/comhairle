import type { EmbeddedSectionBlock } from '@crownshy/api-client/api';
import { HEYFORM_EMBEDDABLE_COMPONENTS } from './heyform/embeddableComponents';
import { POLIS_EMBEDDABLE_COMPONENTS } from './polis/embeddableComponents';

export type SectionBlockMeta<T extends string = string> = {
	type: T;
	label: string;
	description: string;
	/**
	 * Set for a block embedded once per item, such as a HeyForm question. The embed stores the
	 * item in `config.item`, and only embedded items are public.
	 */
	itemName?: { one: string; other: string };
};

export type ReportEmbedProps = {
	toolStepId: string;
	componentType: string;
	/** Which item a per-item block shows, from the embed's `config.item`. */
	item?: string;
	/**
	 * A facilitator is looking before the report is published: the editor, the add-chart
	 * dialog or the report preview. Loads what readers will get from the preview endpoints,
	 * which don't need the report to be published.
	 */
	preview?: boolean;
};

/**
 * Section blocks a facilitator can embed into the conversation report, keyed by the Step's
 * tool type (ADR-0012, ADR-0061). Metadata only, so settings pages can check it without
 * bundling chart components. Adding a tool means one entry here and one in
 * `ReportEmbedLive.svelte`.
 */
const SECTION_BLOCKS_BY_TOOL: Record<string, SectionBlockMeta[]> = {
	polis: POLIS_EMBEDDABLE_COMPONENTS,
	heyform: HEYFORM_EMBEDDABLE_COMPONENTS
};

export function sectionBlocksForTool(toolType: string | null | undefined): SectionBlockMeta[] {
	return (toolType && SECTION_BLOCKS_BY_TOOL[toolType]) || [];
}

export function hasSectionBlocks(toolType: string | null | undefined): boolean {
	return sectionBlocksForTool(toolType).length > 0;
}

/** The tool that owns a section block type, as stored on an embed node. */
export function toolTypeForSectionBlock(componentType: string): string | null {
	for (const [toolType, blocks] of Object.entries(SECTION_BLOCKS_BY_TOOL)) {
		if (blocks.some((block) => block.type === componentType)) return toolType;
	}
	return null;
}

/** A section block the report embeds, with how many of its items for a per-item block. */
export type EmbeddedSectionBlockMeta = SectionBlockMeta & { itemCount: number };

/**
 * The section blocks of one step that the report embeds, in registry order. Only these, and
 * for a per-item block only its embedded items, are public once the report is published
 * (ADR-0061).
 */
export function embeddedSectionBlocksForStep(
	embedded: EmbeddedSectionBlock[],
	stepId: string,
	toolType: string | null | undefined
): EmbeddedSectionBlockMeta[] {
	const ofStep = embedded.filter((sectionBlock) => sectionBlock.workflowStepId === stepId);
	return sectionBlocksForTool(toolType).flatMap((block) => {
		const embeds = ofStep.filter((sectionBlock) => sectionBlock.sectionBlock === block.type);
		if (embeds.length === 0) return [];
		const itemCount = block.itemName
			? new Set(embeds.map((sectionBlock) => sectionBlock.item).filter(Boolean)).size
			: 1;
		return itemCount > 0 ? [{ ...block, itemCount }] : [];
	});
}

/** How a block reads in a list of what's public, such as "Key stats" or "3 questions". */
export function embeddedSectionBlockLabel(block: EmbeddedSectionBlockMeta): string {
	if (!block.itemName) return block.label;
	const noun = block.itemCount === 1 ? block.itemName.one : block.itemName.other;
	return `${block.itemCount} ${noun}`;
}

/** Identifies one embed: step, block and, for a per-item block, the item. */
export function embedKey(sectionBlock: EmbeddedSectionBlock): string {
	return [sectionBlock.workflowStepId, sectionBlock.sectionBlock, sectionBlock.item ?? ''].join(
		':'
	);
}

type DocumentNode = {
	type?: string;
	attrs?: { toolStepId?: unknown; componentType?: unknown; config?: { item?: unknown } };
	content?: DocumentNode[];
};

/**
 * The embeds in one stored report document, so the editor can show what will be public as the
 * facilitator types. Mirrors `tools::public_report_data::embeds` in the API, which decides what
 * is actually served. Text that isn't TipTap JSON has no embeds.
 */
export function sectionBlocksInDocument(document: string): EmbeddedSectionBlock[] {
	let root: DocumentNode;
	try {
		root = JSON.parse(document);
	} catch {
		return [];
	}
	const found: EmbeddedSectionBlock[] = [];
	const visit = (node: DocumentNode | null | undefined) => {
		if (!node || typeof node !== 'object') return;
		const { toolStepId, componentType, config } = node.attrs ?? {};
		if (
			node.type === 'reportComponentEmbed' &&
			typeof toolStepId === 'string' &&
			typeof componentType === 'string'
		) {
			found.push({
				workflowStepId: toolStepId,
				sectionBlock: componentType,
				...(typeof config?.item === 'string' && { item: config.item })
			});
		}
		node.content?.forEach(visit);
	};
	visit(root);
	return found;
}

export type PublicStepResults = {
	stepId: string;
	stepName: string;
	sectionBlocks: EmbeddedSectionBlockMeta[];
};

/** Per step, the section blocks the given report documents embed. Steps with none are left out. */
export function publicResultsByStep(
	documents: string[],
	steps: { id: string; name: string; toolType: string }[]
): PublicStepResults[] {
	const embedded = documents.flatMap(sectionBlocksInDocument);
	return steps
		.map((step) => ({
			stepId: step.id,
			stepName: step.name,
			sectionBlocks: embeddedSectionBlocksForStep(embedded, step.id, step.toolType)
		}))
		.filter((step) => step.sectionBlocks.length > 0);
}
