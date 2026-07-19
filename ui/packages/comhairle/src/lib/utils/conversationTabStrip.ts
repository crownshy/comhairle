/**
 * Shape of the placeholder skeleton for a conversation section's primary sub-tab strip.
 * Cosmetic only: it mimics the real strip closely enough to reserve its height and avoid
 * a layout shift while the strip mounts.
 */
export type TabStripSkeletonShape = {
	/**
	 * Whether the strip's first item leads with an icon + short label (e.g. design's
	 * "Design" tab, events' "All events" tab). Configure's plain sub-tabs do not.
	 */
	leadingIcon: boolean;
	/** Placeholder label widths (in `rem`), in display order, for the tab/name pills. */
	widths: number[];
};

/**
 * Per-section skeletons for the "Row 3" primary strip, shown *while switching into* a section
 * (before the destination's load resolves). Every strip now server-renders from data/constants
 * in the conversation layout (design, configure, invites, events), but the destination strip
 * isn't mounted mid-navigation, so the layout reserves the row with one of these to avoid a
 * shift.
 *
 * Keyed by the section segment right after `/admin/conversations/<id>/`. Widths roughly
 * match each real strip: Configure's four sub-tabs; design/events lead with an icon tab
 * followed by a few variable-width step/event names.
 */
const SECTION_STRIP_SKELETONS: Record<string, TabStripSkeletonShape> = {
	configure: { leadingIcon: false, widths: [3.5, 4, 3, 2.75] },
	design: { leadingIcon: true, widths: [6, 4.5, 7, 5, 5.5] },
	events: { leadingIcon: true, widths: [6, 4.5, 5.5, 5] },
	invites: { leadingIcon: false, widths: [4, 5] }
};

/**
 * The placeholder skeleton shape for the given conversation-admin route's primary sub-tab
 * strip, or `null` if the route has no such strip (so the layout renders nothing).
 *
 * @param pathname - The current `page.url.pathname` (no query string).
 * @param conversationId - The active conversation's id, forming the route base.
 */
export function conversationPrimaryStripSkeleton(
	pathname: string,
	conversationId: string
): TabStripSkeletonShape | null {
	const base = `/admin/conversations/${conversationId}`;
	if (pathname !== base && !pathname.startsWith(`${base}/`)) return null;
	const rest = pathname.slice(base.length).replace(/^\/+/, '');
	const section = rest.split('/')[0];
	return SECTION_STRIP_SKELETONS[section] ?? null;
}
