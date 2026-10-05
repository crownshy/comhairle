import { Megaphone, User } from 'lucide-svelte';
import * as m from '$lib/paraglide/messages';

const seedSource = {
	// The organiser offers seed statements to start discussion, not as views they
	// hold. A check badge would read as an endorsement, so the icon stays neutral.
	icon: Megaphone,
	label: m.polis_seed_statement,
	labelClass: 'border-seed-highlight/50 text-seed-highlight',
	cardClass: 'bg-seed-highlight-bg border-seed-highlight'
};

const participantSource = {
	icon: User,
	label: m.polis_participant_statement,
	labelClass: 'border-participant-highlight/50 text-participant-highlight',
	cardClass: 'bg-participant-highlight-bg border-participant-highlight'
};

export type StatementSource = typeof seedSource;

export function statementSourceOf(isSeed: boolean): StatementSource {
	return isSeed ? seedSource : participantSource;
}
