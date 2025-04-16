import { createApiClient } from '$lib/api/client';
import type { LayoutLoad} from './$types';
import { browser } from '$app/environment';

export const load: LayoutLoad= async ({url, data }) => {
  let token = data.token;
  let user = data.user;
	const api = createApiClient(url.origin+"/api",token, browser ? "client" : "server");
	return {api,user};
};
