import * as m from '$lib/paraglide/messages';

/**
 * The pager pill's label, held in one place because it is unsettled. It reads "Hint"
 * because the Figma does, but what it opens is the full step brief rather than a nudge.
 * "About" and "Help" are the live alternatives. See CONTEXT.md, Hint.
 */
export const stepBriefLabel = () => m.step_brief_hint();
