/** One selectable sub-tab of the Configure page. `id` is the `?tab=` value. */
export type ConfigureTab = { id: string; label: string };

/**
 * Configure sub-tabs, in display order. The first is the default when `?tab=` is absent.
 * Shared so the conversation layout can server-render the strip (Row 3) from this list while
 * the page reads the same list to pick which section to show, keeping the two in lockstep.
 */
export const CONFIGURE_TABS: ConfigureTab[] = [
	{ id: 'details', label: 'Details' },
	{ id: 'content', label: 'Content' },
	{ id: 'access', label: 'Access' },
	{ id: 'team', label: 'Team' }
];
