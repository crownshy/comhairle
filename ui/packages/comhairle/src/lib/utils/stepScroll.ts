/**
 * The participant step route is exactly one screen: the chrome and the pager hold their place
 * and the tool body between them is what scrolls. Tools that used to scroll the document have
 * to ask for that element instead. Outside the step route (admin previews, tests) there is no
 * such element and the document is still the scroller, so every helper falls back to `window`.
 */
export const STEP_SCROLL_ATTRIBUTE = 'data-step-scroll';

export function stepScroller(): HTMLElement | Window {
	if (typeof document === 'undefined') return window;
	return document.querySelector<HTMLElement>(`[${STEP_SCROLL_ATTRIBUTE}]`) ?? window;
}

export function stepScrollTop(scroller: HTMLElement | Window = stepScroller()): number {
	return scroller instanceof Window ? scroller.scrollY : scroller.scrollTop;
}

export function scrollStepToTop(behavior: ScrollBehavior = 'auto') {
	stepScroller().scrollTo({ top: 0, behavior });
}
