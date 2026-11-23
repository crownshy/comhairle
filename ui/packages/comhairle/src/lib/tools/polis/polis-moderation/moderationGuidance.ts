/** Route params for the full moderation guide: when to accept, reject or split, with an example policy. */
export const MODERATION_GUIDE_PARAMS = { guide_id: 'advanced', topic_id: 'moderation-in-depth' };

/** The one rule shown above the queue on every visit. */
export const MODERATION_HEADLINE =
	'Judge whether a statement is votable and safe, not whether you agree with it.';

/** What good moderation looks like, shown in the moderation rules panel. */
export const MODERATION_DO = [
	'Accept views you disagree with, including criticism of the organisers.',
	'Accept blunt or informal writing when the point is clear.',
	'Pick a reason from your policy for every reject, so decisions can be checked later.',
	'Make the same call on the same statement, whoever wrote it and whenever it arrives.',
	'Leave a statement waiting and ask a colleague when you’re not sure.'
];

/** Behaviours that undermine trust in the conversation, shown in the moderation rules panel. */
export const MODERATION_NEVER = [
	'Reject a statement because you disagree with it or find it uncomfortable.',
	'Reword a view to make it softer, stronger, more polite or closer to your own.',
	'Tidy up, summarise or “improve” a statement beyond what voting needs.',
	'Moderate statements you wrote or have a personal stake in.',
	'Hold statements back to slow down a view that’s gaining support.'
];
