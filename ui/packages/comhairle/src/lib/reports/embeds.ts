import type { EmbeddedSectionBlock } from '@crownshy/api-client/api';
import { POLIS_EMBEDDABLE_COMPONENTS } from './polis/embeddableComponents';

export type SectionBlockMeta<T extends string = string> = {
	type: T;
	label: string;
	description: string;
};

export type ReportEmbedProps = {
	toolStepId: string;
	componentType: string;
};

/**
 * Section blocks a facilitator can embed into the conversation report, keyed by the Step's
 * tool type (ADR-0012, ADR-0061). Metadata only, so settings pages can check it without
 * bundling chart components. Adding a tool means one entry here and one in
 * `ReportEmbedLive.svelte`.
 */
const SECTION_BLOCKS_BY_TOOL: Record<string, SectionBlockMeta[]> = {
	polis: POLIS_EMBEDDABLE_COMPONENTS
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

/**
 * The section blocks of one step that the report embeds, in registry order. Only these are
 * public once the report is published (ADR-0061).
 */
export function embeddedSectionBlocksForStep(
	embedded: EmbeddedSectionBlock[],
	stepId: string,
	toolType: string | null | undefined
): SectionBlockMeta[] {
	const embeddedTypes = new Set(
		embedded
			.filter((sectionBlock) => sectionBlock.workflowStepId === stepId)
			.map((sectionBlock) => sectionBlock.sectionBlock)
	);
	return sectionBlocksForTool(toolType).filter((block) => embeddedTypes.has(block.type));
}

type DocumentNode = {
	type?: string;
	attrs?: { toolStepId?: unknown; componentType?: unknown };
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
		const { toolStepId, componentType } = node.attrs ?? {};
		if (
			node.type === 'reportComponentEmbed' &&
			typeof toolStepId === 'string' &&
			typeof componentType === 'string'
		) {
			found.push({ workflowStepId: toolStepId, sectionBlock: componentType });
		}
		node.content?.forEach(visit);
	};
	visit(root);
	return found;
}

export type PublicStepResults = {
	stepId: string;
	stepName: string;
	sectionBlocks: SectionBlockMeta[];
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
