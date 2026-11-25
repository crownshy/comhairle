import type { Tour } from '$lib/tours/types';

/** Route params for the full moderation guide: when to accept, reject or split, with an example policy. */
export const MODERATION_GUIDE_PARAMS = { guide_id: 'advanced', topic_id: 'moderation-in-depth' };

/** The one rule that matters most, opening the moderation rules. */
export const MODERATION_HEADLINE =
	'Judge whether a statement is votable and safe, not whether you agree with it.';

export const MODERATION_HEADLINE_REASON =
	'Moderators decide what participants get to vote on. If people suspect moderation is one-sided, they stop trusting the results.';

export type ModerationRule = { topic: string; do: string; never: string };

/** What good moderation looks like, each paired with the behaviour that undermines trust. */
export const MODERATION_RULES: ModerationRule[] = [
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

/** Shown once, after the rules first close, so moderators know where to open them again. */
export const MODERATION_RULES_POINTER: Tour = {
	id: 'admin-moderation-rules-pointer',
	doneLabel: 'Got it',
	steps: [
		{
			element: '[data-tour="moderation-rules"]',
			title: 'Moderation rules are here any time',
			description: 'Open them again whenever you want a refresher.',
			side: 'bottom',
			align: 'start'
		}
	]
};
