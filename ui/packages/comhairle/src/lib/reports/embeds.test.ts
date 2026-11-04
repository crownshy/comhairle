import { describe, it, expect } from 'vitest';
import {
	embeddedSectionBlocksForStep,
	hasSectionBlocks,
	publicResultsByStep,
	sectionBlocksInDocument,
	sectionBlocksForTool,
	toolTypeForSectionBlock
} from './embeds';
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

describe('embeddedSectionBlocksForStep', () => {
	const STEP_ID = 'step-1';
	const OTHER_STEP_ID = 'step-2';

	it("lists only this step's embedded section blocks, in registry order", () => {
		const embedded = [
			{ workflowStepId: STEP_ID, sectionBlock: 'polis-opinion-groups' },
			{ workflowStepId: OTHER_STEP_ID, sectionBlock: 'polis-area-consensus' },
			{ workflowStepId: STEP_ID, sectionBlock: 'polis-key-stats' }
		];

		expect(
			embeddedSectionBlocksForStep(embedded, STEP_ID, 'polis').map((block) => block.label)
		).toEqual(['Key stats', 'Opinion groups']);
	});

	it("ignores embedded types the step's tool does not own", () => {
		const embedded = [{ workflowStepId: STEP_ID, sectionBlock: 'not-a-section-block' }];

		expect(embeddedSectionBlocksForStep(embedded, STEP_ID, 'polis')).toEqual([]);
	});
});

function embedNode(stepId: string, sectionBlock: string) {
	return {
		type: 'reportComponentEmbed',
		attrs: { toolStepId: stepId, componentType: sectionBlock }
	};
}

describe('sectionBlocksInDocument', () => {
	it('finds embeds at any depth', () => {
		const document = JSON.stringify({
			type: 'doc',
			content: [
				{ type: 'paragraph', content: [{ type: 'text', text: 'Intro' }] },
				embedNode('step-1', 'polis-key-stats'),
				{ type: 'blockquote', content: [embedNode('step-1', 'polis-opinion-groups')] }
			]
		});

		expect(sectionBlocksInDocument(document)).toEqual([
			{ workflowStepId: 'step-1', sectionBlock: 'polis-key-stats' },
			{ workflowStepId: 'step-1', sectionBlock: 'polis-opinion-groups' }
		]);
	});

	it('finds nothing in text that is not TipTap JSON', () => {
		expect(sectionBlocksInDocument('A plain summary')).toEqual([]);
		expect(sectionBlocksInDocument('')).toEqual([]);
	});
});

describe('publicResultsByStep', () => {
	const STEPS = [
		{ id: 'step-1', name: 'Housing priorities', toolType: 'polis' },
		{ id: 'step-2', name: 'Transport', toolType: 'polis' }
	];

	it('groups embeds from every document by step, leaving out steps with none', () => {
		const summary = JSON.stringify({
			type: 'doc',
			content: [embedNode('step-1', 'polis-key-stats')]
		});
		const body = JSON.stringify({
			type: 'doc',
			content: [
				embedNode('step-1', 'polis-opinion-groups'),
				embedNode('step-1', 'polis-key-stats')
			]
		});

		expect(publicResultsByStep([summary, body], STEPS)).toEqual([
			{
				stepId: 'step-1',
				stepName: 'Housing priorities',
				sectionBlocks: sectionBlocksForTool('polis').filter((block) =>
					['polis-key-stats', 'polis-opinion-groups'].includes(block.type)
				)
			}
		]);
	});
});
