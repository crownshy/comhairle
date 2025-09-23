import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ parent }) => {
	let { conversation, workflows, workflow_steps } = await parent()
	return { conversation, workflows, workflow_steps }
}
