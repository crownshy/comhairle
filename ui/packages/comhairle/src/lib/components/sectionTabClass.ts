import { cn } from '$lib/utils';

/**
 * Classes for one tab in a section's tab strip (the row under the section header): an
 * underline marks the active tab. Shared by every strip so they look the same.
 *
 * @param active - Whether this tab is the current page.
 * @param className - Extra classes for this tab.
 */
export function sectionTabClass(active: boolean, className?: string): string {
	return cn(
		'focus-inset inline-flex h-11 items-center gap-1.5 border-b-2 -mx-2 px-2 text-base font-medium whitespace-nowrap transition-colors',
		active
			? 'border-primary text-foreground'
			: 'text-muted-foreground hover:text-foreground border-transparent',
		className
	);
}

/**
 * Classes for one tab in a sub-tab strip, the lighter row under a section's own tabs.
 *
 * @param active - Whether this tab is the current page.
 */
export function subTabClass(active: boolean): string {
	return cn(
		'focus-inset text-foreground inline-flex h-10 items-center px-3.5 -mx-2 px-2 text-base font-medium whitespace-nowrap transition-opacity',
		active ? 'text-primary' : 'opacity-70 hover:opacity-100'
	);
}

/** Sub-tabs carry `px-3.5`, so their list shifts left by the same amount to put the first label on the column edge. */
export const SUB_TAB_LIST_OFFSET = 'relative right-3.5';
