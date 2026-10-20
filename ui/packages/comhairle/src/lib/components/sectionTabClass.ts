/**
 * Classes for one tab in a section's tab strip (the row under the section header): an
 * underline marks the active tab. Shared by every strip so they look the same.
 *
 * @param active - Whether this tab is the current page.
 */
export function sectionTabClass(active: boolean): string {
	const base =
		'inline-flex h-11 items-center gap-1.5 border-b-2 text-base font-medium whitespace-nowrap transition-colors';
	return active
		? `${base} border-primary text-foreground`
		: `${base} text-muted-foreground hover:text-foreground border-transparent`;
}
