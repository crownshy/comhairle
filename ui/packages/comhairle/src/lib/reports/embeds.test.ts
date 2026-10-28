import { describe, it, expect } from 'vitest';
import { hasSectionBlocks, sectionBlocksForTool, toolTypeForSectionBlock } from './embeds';
import { POLIS_EMBEDDABLE_COMPONENTS } from './polis/embeddableComponents';

describe('sectionBlocksForTool', () => {
	it('lists the Polis section blocks', () => {
		expect(sectionBlocksForTool('polis')).toBe(POLIS_EMBEDDABLE_COMPONENTS);
	});

	it('is empty for a tool with no section blocks or no tool at all', () => {
		expect(sectionBlocksForTool('learn')).toEqual([]);
		expect(sectionBlocksForTool(null)).toEqual([]);
		expect(sectionBlocksForTool(undefined)).toEqual([]);
	});
});

describe('hasSectionBlocks', () => {
	it('is true only for tools with section blocks', () => {
		expect(hasSectionBlocks('polis')).toBe(true);
		expect(hasSectionBlocks('learn')).toBe(false);
		expect(hasSectionBlocks(null)).toBe(false);
	});
});

describe('toolTypeForSectionBlock', () => {
	it('finds the tool that owns a stored section block type', () => {
		expect(toolTypeForSectionBlock('polis-key-stats')).toBe('polis');
	});

	it('is null for a type no tool owns', () => {
		expect(toolTypeForSectionBlock('not-a-section-block')).toBeNull();
	});
});
