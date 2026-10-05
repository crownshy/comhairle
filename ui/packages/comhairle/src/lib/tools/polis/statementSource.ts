import * as m from '$lib/paraglide/messages';

const seedSource = {
	label: m.polis_seed_statement,
	labelClass: 'text-seed-highlight',
	cardClass: 'bg-seed-highlight-bg border-seed-highlight'
};

const participantSource = {
	label: m.polis_participant_statement,
	labelClass: 'text-participant-highlight',
	cardClass: 'bg-participant-highlight-bg border-participant-highlight'
};

export type StatementSource = typeof seedSource;

export function statementSourceOf(isSeed: boolean): StatementSource {
	return isSeed ? seedSource : participantSource;
}
