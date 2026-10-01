import type {
	ConversationAction,
	OrganizationAction,
	PermissionResourceType,
	SystemAction,
	UserAction,
	UserActions
} from '@crownshy/api-client/api';
import type { UserRoles } from '@crownshy/api-client/api';
import type { createApiClient } from '@crownshy/api-client/client';
import { tryCatchAsync } from './errorHandling';

export const SYSTEM_RESOURCE_ID = '00000000-0000-0000-0000-000000000000';

export function canAccessAdminPortal(roles: readonly UserRoles[] | null | undefined): boolean {
	return (
		roles?.some(
			(role) =>
				role.resource === 'Site' &&
				(role.roles.includes('Admin') || role.roles.includes('SuperAdmin'))
		) ?? false
	);
}

type ResourceActions = {
	conversation: ConversationAction;
	organization: OrganizationAction;
	system: SystemAction;
};

export function canPerformAction(
	permissions: UserActions | null | undefined,
	action: UserAction
): boolean {
	return permissions?.actions.includes(action) ?? false;
}

export function createPermissions(getResources: () => readonly (UserActions | null | undefined)[]) {
	return {
		can<Resource extends PermissionResourceType>(
			resourceType: Resource,
			action: ResourceActions[Resource],
			resourceId: string = SYSTEM_RESOURCE_ID
		): boolean {
			return getResources().some(
				(resource) =>
					resource?.resourceType === resourceType &&
					resource.resourceId === resourceId &&
					canPerformAction(resource, action)
			);
		}
	};
}

export async function loadUserActions(
	api: Pick<ReturnType<typeof createApiClient>, 'GetUserActions'>,
	resourceType: PermissionResourceType,
	resourceId: string = SYSTEM_RESOURCE_ID
): Promise<UserActions | null> {
	const result = await tryCatchAsync(() =>
		api.GetUserActions({
			params: { resource_type: resourceType, resource_id: resourceId }
		})
	);
	if (result.err !== null) return null;
	if (result.ok.resourceType !== resourceType || result.ok.resourceId !== resourceId) return null;
	return result.ok;
}
