import type { PageLoad } from './$types';
import { tryCatchAsync, type ErrorType, type Result } from '$lib/utils/errorHandling';
import { typed } from '$lib/utils/types';
import type { OrganizationDto } from '@crownshy/api-client/api';
import BinaryTree from '$lib/data-structures/BinaryTree';

type CohostOrganizations = { id: string; name: string };

type AccessData = {
	canManageCohosts: boolean;
	streamedCohostOrganizations: Promise<Result<'ok', CohostOrganizations[], ErrorType>>;
};

export const load: PageLoad = async ({ parent, params }) => {
	const { api, user, conversation } = await parent();

	const canManageCohosts = user.id === conversation.ownerId;
	if (!canManageCohosts) {
		return typed<AccessData>({
			canManageCohosts: false,
			streamedCohostOrganizations: tryCatchAsync<CohostOrganizations[], ErrorType>(
				async (ok) => ok<CohostOrganizations[]>([])
			)
		});
	}

	return typed<AccessData>({
		canManageCohosts: true,
		streamedCohostOrganizations: tryCatchAsync<CohostOrganizations[], ErrorType>(
			async (ok, err) => {
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

				const binaryTree = new BinaryTree<string, CohostOrganizations>();

				for (const organization of organizations.ok) {
					if (
						organization.id === conversation.organizationId ||
						cohostOrganizationIds.includes(organization.id)
					) {
						continue;
					}

					binaryTree.insert(organization.name, {
						id: organization.id,
						name: organization.name
					});
				}

				return ok(binaryTree.toArray());
			}
		)
	});
};
