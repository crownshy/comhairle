import * as m from '$lib/paraglide/messages';
import type { Tour } from '$lib/tours/types';

/**
 * One run through everything on the screen, in the order a thumb would find it: the two
 * corners at the top, the assistant in the body, the menu it also lives in, then the two
 * corners at the bottom (ADR-0026, ADR-0034). The captions name their place in words rather
 * than relying on the ring, so they carry the same meaning read aloud.
 *
 * A beat whose control is not on this screen is dropped, and the count is of what is left.
 *
 * The assistant is the one beat that cannot be settled by looking: it mounts a moment after
 * the step body does, so the caller passes what the page already knows about it rather than
 * making the tour wait and find out.
 */
export function stepTour({ assistant }: { assistant: boolean }): Tour {
	return {
		id: 'participant-step',
		stops: [
			{ target: 'intro', text: () => m.step_tour_before_you_start() },
			{ target: 'brief', text: () => m.step_tour_brief() },
			...(assistant
				? [
						{
							target: 'assistant',
							text: () => m.step_tour_assistant(),
							mountsLate: true
						}
					]
				: []),
			{
				target: 'menu',
				text: () => m.step_tour_assistant_later(),
				side: 'bottom' as const,
				align: 'end' as const
			},
			{ target: 'back', text: () => m.step_tour_back() },
			{ target: 'forward', text: () => m.step_tour_forward() }
		]
	};
}

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
