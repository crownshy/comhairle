import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, user, workflows, workflowSteps, preview, sealed } = await parent();

	// A sealed participant has nothing to return to, so the page's revisit section drops out
	// on its own (it renders only when this list is non-empty). Filtering here rather than in
	// the markup keeps this page and the step gate reading the same flag.
	const revisitableSteps = sealed
		? []
		: workflowSteps.filter((s) => s.canRevisit).sort((a, b) => a.stepOrder - b.stepOrder);

	return { conversation, user, workflow: workflows[0], revisitableSteps, preview };
};
