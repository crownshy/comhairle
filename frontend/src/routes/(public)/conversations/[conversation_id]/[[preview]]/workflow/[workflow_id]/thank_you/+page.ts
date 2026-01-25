import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ parent }) => {
	let { conversation, user, workflows } = await parent()
	return { conversation, user, workflow: workflows[0] }
}
