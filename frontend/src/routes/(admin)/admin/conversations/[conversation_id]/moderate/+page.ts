import type { PageLoad } from "./$types"


export const load: PageLoad = async ({ params, parent }) => {
	let { api, conversation } = await parent();

	return { invites, conversation };
}
