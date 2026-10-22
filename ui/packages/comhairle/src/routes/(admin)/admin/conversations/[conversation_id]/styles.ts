/**
 * The region under the section header and tab strips. From md up it scrolls on its own so they
 * stay put. `relative` keeps absolute children (e.g. a switch's hidden input) inside it; without
 * it they anchor to `main` and give the page a second scrollbar.
 */
export const SCROLL_REGION_CLASS = 'relative grow md:min-h-0 md:overflow-y-auto';

/** The padded backdrop a workflow step page sits on, shared with its switching skeleton. */
export const STEP_PAGE_CLASS = 'bg-admin-background pt-page-top px-gutter pb-8 sm:pb-12';
