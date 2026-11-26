import type { JourneyStepVisit, ParticipantJourney } from './sampleMonitorData';

/** Shared by the journey bars and the selected-journey detail so their colours always match. */

export function isFinished(journey: ParticipantJourney): boolean {
	return journey.visits.every((visit) => visit.status === 'done');
}

/** Background class for one step's segment. */
export function segmentClass(
	journey: ParticipantJourney,
	visit: JourneyStepVisit,
	index: number
): string {
	if (visit.status === 'left') return 'bg-amber-500';
	// Reached the end: the last step turns green.
	if (isFinished(journey) && index === journey.visits.length - 1) return 'bg-green-500';
	if (visit.status === 'in_progress') return 'bg-primary';
	// Finished steps are muted so the live (pulsing) step and drop-offs stand out.
	// Two greys alternate so neighbouring steps stay distinguishable.
	return index % 2 === 0 ? 'bg-muted-foreground/45' : 'bg-muted-foreground/25';
}

/** Adds minutes to a "HH:MM" clock time. */
export function addMinutes(clock: string, minutes: number): string {
	const [hours, mins] = clock.split(':').map(Number);
	const total = hours * 60 + mins + minutes;
	return `${String(Math.floor(total / 60) % 24).padStart(2, '0')}:${String(total % 60).padStart(2, '0')}`;
}
