/**
 * Event-detail sub-tabs, in display order. `value` is the `?subtab=` value; the first is the
 * default. Shared so the conversation layout can server-render the strip (Row 4) from this
 * list while the event page reads `?subtab=` to pick which section to show.
 */
export const EVENT_SUBTABS: { label: string; value: string }[] = [
	{ label: 'Details', value: 'details' },
	{ label: 'Event Structure', value: 'structure' },
	{ label: 'Facilitators', value: 'facilitators' },
	{ label: 'Location', value: 'location' },
	{ label: 'Invites', value: 'invites' },
	{ label: 'Recordings', value: 'recordings' }
];
