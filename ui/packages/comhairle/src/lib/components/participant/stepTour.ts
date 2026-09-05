import * as m from '$lib/paraglide/messages';
import type { Tour } from '$lib/tours/types';

/**
 * The chrome's fixed places, circled one at a time on the first run through a conversation
 * (ADR-0026). The captions name their place in words rather than relying on the ring, so
 * they carry the same meaning read aloud.
 */
export const stepTour: Tour = {
	id: 'participant-step',
	stops: [
		{ target: 'intro', text: () => m.step_tour_before_you_start() },
		{ target: 'brief', text: () => m.step_tour_brief() },
		{ target: 'back', text: () => m.step_tour_back() },
		{ target: 'forward', text: () => m.step_tour_forward() }
	]
};

type ProgressLike = { progressStatus?: string | null };

/**
 * Whether this looks like a first time through: the participant has joined and nothing is
 * finished yet. Read from progress rather than from a flag of our own, so someone who comes
 * back to an unfinished conversation in another browser is still offered it, and someone who
 * has already completed a step never is.
 */
export function isFirstRun(steps: ProgressLike[] | null | undefined): boolean {
	if (!steps?.length) return false;
	return !steps.some((step) => step.progressStatus === 'done');
}
