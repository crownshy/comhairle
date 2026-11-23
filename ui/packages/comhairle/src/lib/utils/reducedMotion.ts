/** Smooth scrolling, or a jump when the viewer prefers reduced motion. */
export function scrollBehavior(): ScrollBehavior {
	return window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth';
}
