import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ parent }) => {
	let { conversations } = await parent()
	console.log("conversations ", conversations)
	return { conversations }
}
