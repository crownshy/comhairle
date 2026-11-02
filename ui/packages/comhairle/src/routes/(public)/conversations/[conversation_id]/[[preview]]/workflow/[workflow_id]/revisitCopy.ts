import { m } from '$lib/paraglide/messages';
import type { ToolType } from '$lib/tool_meta';

type RevisitCopy = {
	/** The kind of step, as participants know it. */
	label: () => string;
	/** What they can still do there. */
	body: () => string;
	/** The button that takes them back. */
	action: () => string;
};

/** Copy for each tool's card in the "Go back to a step" list. */
export const REVISIT_COPY: Record<ToolType, RevisitCopy> = {
	learn: {
		label: m.revisit_step_learn_label,
		body: m.revisit_step_learn_body,
		action: m.revisit_step_learn_action
	},
	polis: {
		label: m.revisit_step_polis_label,
		body: m.revisit_step_polis_body,
		action: m.revisit_step_polis_action
	},
	thinkingspace: {
		label: m.revisit_step_thinkingspace_label,
		body: m.revisit_step_thinkingspace_body,
		action: m.revisit_step_thinkingspace_action
	},
	heyform: {
		label: m.revisit_step_heyform_label,
		body: m.revisit_step_heyform_body,
		action: m.revisit_step_heyform_action
	},
	prioritization: {
		label: m.revisit_step_prioritization_label,
		body: m.revisit_step_prioritization_body,
		action: m.revisit_step_prioritization_action
	},
	elicitationbot: {
		label: m.revisit_step_elicitationbot_label,
		body: m.revisit_step_elicitationbot_body,
		action: m.revisit_step_elicitationbot_action
	},
	stories: {
		label: m.revisit_step_stories_label,
		body: m.revisit_step_stories_body,
		action: m.revisit_step_stories_action
	}
};
