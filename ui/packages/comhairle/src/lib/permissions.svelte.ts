import { page } from '$app/state';
import { createPermissions } from '$lib/utils/permissions';

export const permissions = createPermissions(() => [
	page.data.systemActions,
	page.data.conversationActions,
	page.data.organizationActions,
	...(page.data.resourceActions ?? [])
]);
