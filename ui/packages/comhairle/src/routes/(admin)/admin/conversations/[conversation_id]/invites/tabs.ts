/**
 * Recruit (invites) sub-tabs, in display order. `value` is the `?subtab=` value; the first
 * is the default. Shared so the conversation layout can server-render the strip (Row 3) from
 * this list while the page reads `?subtab=` to pick which section to show.
 */
export const INVITE_SUBTABS: { label: string; value: string }[] = [
	{ label: 'Email', value: 'email' },
	{ label: 'Open Links', value: 'open-links' },
	{ label: 'Physical', value: 'physical' }
];
