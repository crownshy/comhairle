import { redirect } from "@sveltejs/kit"
import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ parent }) => {
	let { user } = await parent()
	if (!user) {
		redirect(307, "/")
	}

}
