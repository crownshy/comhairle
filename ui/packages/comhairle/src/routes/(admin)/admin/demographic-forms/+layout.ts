import { error } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { HttpStatus } from '$lib/utils/constants';

type Organization = {
	id: string;
	name: string;
};

export const load: LayoutLoad = async ({ parent, depends, url }) => {
	depends(key('admin/demographic-forms'));
	const { api, userOrganizations } = await parent();

	const organizations: Organization[] = [];

	for (const userOrg of userOrganizations.organizations) {
		if (userOrg.isAssociated) {
			organizations.push({
				id: userOrg.organization.id,
				name: userOrg.organization.name
			});
		}
	}

	const requestedId = url.searchParams.get('organization');
	const organization = organizations.find((org) => org.id === requestedId) ?? null;

	if (organization === null) {
		return {
			organizations,
			organizationId: null,
			canEdit: false,
			questions: [],
			forms: [],
			tags: []
		};
	}

	const formsResponse = tryCatchAsync(() =>
		api
			.ListDemographicFormsLatest({ queries: { organization: organization.id } })
			.then((form) => form.filter((f) => !f.archived_at))
	);

	const questionsResponse = tryCatchAsync(() =>
		api
			.ListDemographicQuestionsLatest({ queries: { organization: organization.id } })
			.then((question) => question.filter((q) => !q.archived_at))
	);

	const tagsResponse = tryCatchAsync(() =>
		api.ListDemographicTags({ queries: { organization: organization.id } })
	);

	const forms = await formsResponse;
	if (forms.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load forms');
	}

	const questions = await questionsResponse;
	if (questions.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load questions');
	}

	const tags = await tagsResponse;
	if (tags.err !== null) {
		error(HttpStatus.InternalServerError, 'Could not load tags');
	}

	return {
		organizations,
		organizationId: organization.id,
		canEdit: true,
		forms: forms.ok,
		questions: questions.ok,
		tags: tags.ok
	};
};
