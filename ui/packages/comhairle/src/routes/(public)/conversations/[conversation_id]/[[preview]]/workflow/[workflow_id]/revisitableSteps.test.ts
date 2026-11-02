import { describe, expect, it } from 'vitest';
import type { LocalizedWorkflowStepDto } from '@crownshy/api-client/api';
import { revisitableSteps, stepToolType } from './revisitableSteps';

function step(overrides: Partial<LocalizedWorkflowStepDto>): LocalizedWorkflowStepDto {
	return {
		canRevisit: true,
		stepOrder: 1,
		toolConfig: null,
		previewToolConfig: { type: 'learn' },
		...overrides
	} as LocalizedWorkflowStepDto;
}

describe('revisitableSteps', () => {
	it('keeps only revisitable steps, in step order', () => {
		const steps = [
			step({ id: 'third', stepOrder: 3 }),
			step({ id: 'locked', stepOrder: 2, canRevisit: false }),
			step({ id: 'first', stepOrder: 1 })
		];
		expect(revisitableSteps(steps).map((s) => s.id)).toEqual(['first', 'third']);
	});
});

describe('stepToolType', () => {
	it('reads the live config once the conversation is launched', () => {
		const launched = step({
			toolConfig: { type: 'polis' } as LocalizedWorkflowStepDto['toolConfig'],
			previewToolConfig: { type: 'learn' } as LocalizedWorkflowStepDto['previewToolConfig']
		});
		expect(stepToolType(launched)).toBe('polis');
	});

	it('falls back to the draft config before launch', () => {
		expect(stepToolType(step({ toolConfig: null }))).toBe('learn');
	});
});
