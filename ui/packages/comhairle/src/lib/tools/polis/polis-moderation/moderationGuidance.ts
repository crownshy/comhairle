/** Route params for the full moderation guide: when to accept, reject or split, with an example policy. */
export const MODERATION_GUIDE_PARAMS = { guide_id: 'advanced', topic_id: 'moderation-in-depth' };

/**
 * Query param marking a visit from the moderator email, which opens the rules. The email's
 * link is built in api/src/worker_service/wiki_poll_moderation_notify.rs and must match.
 */
export const FROM_PARAM = 'from';
export const FROM_EMAIL = 'email';

/** The one rule moderators should remember. */
export const MODERATION_HEADLINE =
	'Judge whether a statement is votable and safe, not whether you agree with it.';

/** Why the rule matters, shown under the headline. */
export const MODERATION_WHY =
	'Moderators decide what participants get to vote on. If people suspect moderation is one-sided, they stop trusting the results.';

/** Each "do" next to the "never" it guards against, so moderators read them as one idea. */
export const MODERATION_PAIRS: { topic: string; do: string; never: string }[] = [
	{
		topic: 'Views you disagree with',
		do: 'Accept views you disagree with, including criticism of the organisers.',
		never: 'Reject a statement because you disagree with it or find it uncomfortable.'
	},
	{
		topic: 'How it’s written',
		do: 'Accept blunt or informal writing when the point is clear.',
		never: 'Reword a view to make it softer, stronger, more polite or closer to your own.'
	},
	{
		topic: 'Editing',
		do: 'Change only what voting needs, like splitting a statement that makes two points.',
		never: 'Tidy up, summarise or “improve” a statement beyond what voting needs.'
	},
	{
		topic: 'Being consistent',
		do: 'Make the same call on the same statement, whoever wrote it and whenever it arrives.',
		never: 'Hold statements back to slow down a view that’s gaining support.'
	},
	{
		topic: 'When it’s not your call',
		do: 'Leave a statement waiting and ask a colleague when you’re not sure.',
		never: 'Moderate statements you wrote or have a personal stake in.'
	}
];

export type PracticeChoice = 'accept' | 'reject';

export type PracticeStatement = {
	text: string;
	answer: PracticeChoice;
	/** The reason label a reject should use, when the answer is reject. */
	reason?: string;
	explanation: string;
};

/** Made-up statements for the practice area. Nothing here reaches Polis. */
export const PRACTICE_STATEMENTS: PracticeStatement[] = [
	{
		text: 'The organisers have ignored the queues for years. It’s a disgrace and they should be embarrassed.',
		answer: 'accept',
		explanation:
			'Blunt criticism of the organisers is still a clear view people can vote on. Accept it as written.'
	},
	{
		text: 'Dave Morris who runs the dodgems is always rude to kids. Someone should sack him.',
		answer: 'reject',
		reason: 'Privacy or personal info',
		explanation:
			'It names a private person. A statement about staff being rude, without a name, would be fine to accept.'
	},
	{
		text: 'Get 50% off your entry at cheap-park-tickets.biz, today only!',
		answer: 'reject',
		reason: 'Advertising or campaigning',
		explanation: 'This is advertising, not a view about the park.'
	}
];
