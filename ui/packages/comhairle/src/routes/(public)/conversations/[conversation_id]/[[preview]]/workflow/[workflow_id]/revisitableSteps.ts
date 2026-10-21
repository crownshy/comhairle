import type { LocalizedWorkflowStepDto } from '@crownshy/api-client/api';
import type { ToolType } from '$lib/tool_meta';

type RevisitStep = Pick<
	LocalizedWorkflowStepDto,
	'canRevisit' | 'stepOrder' | 'toolConfig' | 'previewToolConfig'
>;

/** The steps a finished participant can go back to, in the order they took them. */
export function revisitableSteps<Step extends RevisitStep>(steps: Step[]): Step[] {
	return steps.filter((step) => step.canRevisit).sort((a, b) => a.stepOrder - b.stepOrder);
}

/** The tool a step runs. The live config is empty until launch, so preview reads the draft. */
export function stepToolType(step: RevisitStep): ToolType {
	return (step.toolConfig ?? step.previewToolConfig).type;
}
