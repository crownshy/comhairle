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
