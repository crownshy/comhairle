import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const {
		conversation,
		user,
		workflows,
		workflowSteps,
		preview,
		sealed,
		availableDocuments,
		hasKnowledgeBaseDocs
	} = await parent();

	const steps = [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder);

	// A sealed participant has nothing to return to, so the page's revisit section drops out
	// on its own (it renders only when this list is non-empty). Filtering here rather than in
	// the markup keeps this page and the step gate reading the same flag.
	const revisitableSteps = sealed ? [] : steps.filter((s) => s.canRevisit);

	return {
		conversation,
		user,
		workflow: workflows[0],
		steps,
		revisitableSteps,
		preview,
		availableDocuments,
		hasKnowledgeBaseDocs,
		// The end of the flow keeps the flow's chrome, which carries its own preview marker.
		// Flags off the conversation layout's full-width banner, as the step pages do.
		participantChrome: true
	};
};
