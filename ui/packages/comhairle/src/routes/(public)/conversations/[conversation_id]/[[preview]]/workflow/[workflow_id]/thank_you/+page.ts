import type { PageLoad } from './$types';
import type { ApiError, ParticipationCountResponse } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('public/participation-count'));
	const { api, conversation, user, workflows, workflowSteps, preview, sealed } = await parent();

	// The count is a nice-to-have, so a failed fetch hides it rather than breaking the page.
	// Preview skips it because the only people counted there are staff test runs.
	let participantCount: number | null = null;
	if (!preview) {
		const countResult = await tryCatchAsync<ParticipationCountResponse, ApiError>(() =>
			api.GetParticipationCount({ params: { conversation_id: conversation.id } })
		);
		if (countResult.err === null) participantCount = countResult.ok.participantCount;
	}

	// A sealed participant has nothing to return to, so the page's revisit section drops out
	// on its own (it renders only when this list is non-empty). Filtering here rather than in
	// the markup keeps this page and the step gate reading the same flag.
	const revisitableSteps = sealed
		? []
		: workflowSteps.filter((s) => s.canRevisit).sort((a, b) => a.stepOrder - b.stepOrder);

	return {
		conversation,
		user,
		workflow: workflows[0],
		revisitableSteps,
		preview,
		participantCount
	};
};
