import type { PageLoad } from "./$types";

export const load: PageLoad = async (event) => {
	let step_id = event.params.step_id;
	let { conversation, workflows, workflow_steps } = await event.parent()
	return { step_id, conversation, workflows, workflow_steps }
}

