
import type { PageLoad } from "./$types";

export const load: PageLoad = async (event) => {
	const step_id = event.params.step_id;
	const { conversation, workflows, workflowSteps } = await event.parent()
	return { step_id, conversation, workflows, workflowSteps }
}

