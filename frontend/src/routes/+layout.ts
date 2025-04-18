import { createApiClient } from '$lib/api/client';
import type { LayoutLoad} from './$types';
import { browser } from '$app/environment';

export const load: LayoutLoad= async ({url, data }) => {
  console.log("LAYOUT TS")
  let token = data.token;
  let user = data.user;
	const api = createApiClient(url.origin+"/api",token, browser ? "client" : "server");
	console.log("Retrugning ",{user})
	return {api,user};
};
