/**
 * Live step-brief preview: the admin's description editor pushes its unsaved content into
 * the preview iframe so slide breaks appear as they are typed.
 *
 * `description` has no draft/published split on the step DTO (ADR-0017), so the preview
 * route can only show unsaved text if it is handed to it directly. Same origin, so a plain
 * postMessage is enough.
 */
export const STEP_PREVIEW_MESSAGE = 'comhairle:step-brief-preview';

export type StepPreviewMessage = {
	type: typeof STEP_PREVIEW_MESSAGE;
	stepId: string;
	description: string;
};

export function postStepPreview(
	target: Window | null | undefined,
	stepId: string,
	description: string
) {
	if (!target) return;
	const message: StepPreviewMessage = { type: STEP_PREVIEW_MESSAGE, stepId, description };
	target.postMessage(message, window.location.origin);
}

/**
 * Subscribes to draft descriptions for one step. Returns the unsubscribe function.
 *
 * Messages from any other origin are ignored: this only ever carries content the admin is
 * already authorised to edit, but a preview page is embeddable and should not accept
 * content from whoever embedded it.
 */
export function onStepPreview(stepId: string, apply: (description: string) => void): () => void {
	function handle(event: MessageEvent) {
		if (event.origin !== window.location.origin) return;
		const data = event.data as StepPreviewMessage | undefined;
		if (data?.type !== STEP_PREVIEW_MESSAGE) return;
		if (data.stepId !== stepId) return;
		apply(data.description);
	}

	window.addEventListener('message', handle);
	return () => window.removeEventListener('message', handle);
}
