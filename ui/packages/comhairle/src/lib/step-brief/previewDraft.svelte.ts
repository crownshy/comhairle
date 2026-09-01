/**
 * The step description as it is being typed, shared because the editor and the preview
 * trigger live in different components: the editor is the Configure page, the trigger sits
 * in the step layout's sub-tab strip so every sub-tab can open the preview.
 *
 * Keyed by step id so a draft left behind by one step never posts into another step's
 * preview. Nothing outside the step shell needs it, so it stays a module, not a context.
 */
let draftStepId = $state<string | null>(null);
let draftDescription = $state('');

export const stepPreviewDraft = {
	/** The unsaved description for `stepId`, or undefined when nothing has been typed for it. */
	for(stepId: string): string | undefined {
		return draftStepId === stepId ? draftDescription : undefined;
	},
	set(stepId: string, description: string) {
		draftStepId = stepId;
		draftDescription = description;
	}
};
