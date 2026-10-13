export const STEP_SCROLL_ATTRIBUTE = 'data-step-scroll';

/**
 * The participant step's scroll container, or the window where there is none (admin
 * previews, tests).
 */
export function stepScroller(): HTMLElement | Window {
	if (typeof document === 'undefined') return window;
	return document.querySelector<HTMLElement>(`[${STEP_SCROLL_ATTRIBUTE}]`) ?? window;
}

export function stepScrollTop(scroller: HTMLElement | Window = stepScroller()): number {
	return scroller instanceof Window ? scroller.scrollY : scroller.scrollTop;
}

/** How far `element`'s top edge sits from the top of the step's scrolled content. */
export function offsetInStepScroll(
	element: HTMLElement,
	scroller: HTMLElement | Window = stepScroller()
): number {
	const scrollerTop = scroller instanceof Window ? 0 : scroller.getBoundingClientRect().top;
	return stepScrollTop(scroller) + element.getBoundingClientRect().top - scrollerTop;
}

/** Smooth scrolling falls back to a jump when the viewer prefers reduced motion. */
export function scrollStepTo(top: number, { smooth = false } = {}) {
	const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
	stepScroller().scrollTo({ top, behavior: smooth && !reduceMotion ? 'smooth' : 'auto' });
}

export function scrollStepToTop(options: { smooth?: boolean } = {}) {
	scrollStepTo(0, options);
}
