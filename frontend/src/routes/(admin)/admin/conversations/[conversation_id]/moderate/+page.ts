import type { PageLoad } from "./$types"


export const load: PageLoad = async ({ params, parent }) => {
	let { api, conversation, workflow_steps } = await parent();

	return { conversation, workflow_steps };
}
