import type { PageLoad } from './$types';
import { tryCatchAsync, type ErrorType, type Result } from '$lib/utils/errorHandling';
import { typed } from '$lib/utils/types';
import type { OrganizationDto, OrganizationWithPermissionDto } from '@crownshy/api-client/api';
import BinaryTree from '$lib/data-structures/BinaryTree';

type CohostOrganization = OrganizationWithPermissionDto;

type AccessData = {
	canManageCohosts: boolean;
	streamedCohostOrganizations: Promise<Result<'ok', CohostOrganization[], ErrorType>>;
};

export const load: PageLoad = async ({ parent, params }) => {
	const { api, user, conversation } = await parent();

	const canManageCohosts = user.id === conversation.ownerId;
	if (!canManageCohosts) {
		return typed<AccessData>({
			canManageCohosts: false,
			streamedCohostOrganizations: tryCatchAsync<CohostOrganization[], ErrorType>(
				async (ok) => ok<CohostOrganization[]>([])
			)
		});
	}

	return typed<AccessData>({
		canManageCohosts: true,
		streamedCohostOrganizations: tryCatchAsync<CohostOrganization[], ErrorType>(
			async (ok, err) => {
				// NOTE: This logic should maybe be moved to the backend
				const organizations = await tryCatchAsync(() =>
					api
						.ListOrganizations({
							queries: { limit: 500 }
						})
						.then((result) => result.records)
				);

				if (organizations.err !== null) {
					throw err(organizations.err);
				}

				const cohostOrganizations = await tryCatchAsync(() =>
					api.ListConversationCoHostOrganizations({
						params: { conversation_id: params.conversation_id }
					})
				);

				if (cohostOrganizations.err !== null) {
					throw err(cohostOrganizations.err);
				}

				const cohostOrganizationIds: OrganizationDto['id'][] = cohostOrganizations.ok.map(
					(c) => c.id
				);

				const binaryTree = new BinaryTree<string, CohostOrganization>();

				for (const organization of organizations.ok) {
					if (
						organization.id === conversation.organizationId ||
						cohostOrganizationIds.includes(organization.id)
					) {
						continue;
					}

					const cohostOrganization = cohostOrganizations.ok.find(
						(c) => c.id === organization.id
					);
					if (!cohostOrganization) {
						continue;
					}

					binaryTree.insert(cohostOrganization.name, cohostOrganization);
				}

				return ok(binaryTree.toArray());
			}
		)
	});
};
