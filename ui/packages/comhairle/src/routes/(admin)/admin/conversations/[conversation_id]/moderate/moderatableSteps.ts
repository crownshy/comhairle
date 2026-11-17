type StepWithToolConfigs = {
	toolConfig?: { type: string } | null;
	previewToolConfig?: { type: string } | null;
};

/** Whether a step has something to moderate: a Participant-led poll, live or preview to match the conversation. */
export function isModeratableStep(isLive: boolean, step: StepWithToolConfigs): boolean {
	const toolConfig = isLive ? step.toolConfig : step.previewToolConfig;
	return toolConfig?.type === 'polis';
}
