/** The bar a pager sits in at the bottom of the screen: see-through, blurred, a hairline on top. */
export const PAGER_BAR_SURFACE_CLASS =
	'bg-background/70 border-border/40 border-t backdrop-blur-lg';

/** The pager's row: back in one corner, forward in the other (ADR-0048). */
export const PAGER_ROW_CLASS = 'flex h-20 items-center gap-2';

export const PAGER_BUTTON_CLASS =
	'text-foreground inline-flex items-center gap-1 transition-transform active:scale-90 disabled:opacity-30 aria-disabled:opacity-30 motion-reduce:transition-none motion-reduce:active:scale-100';
