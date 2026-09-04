import { cubicOut } from 'svelte/easing';
import type { FlyParams } from 'svelte/transition';
import { prefersReducedMotion } from '$lib/utils/reducedMotion';

/** 1 turns the page forward, -1 back. */
export type SlideDirection = 1 | -1;

/**
 * The incoming slide arrives from the side the reader is heading towards, so a page turn
 * reads as a turn and not a swap. Reduced motion collapses it to an instant swap.
 */
export function slideIn(direction: SlideDirection): FlyParams {
	if (prefersReducedMotion()) return { duration: 0 };
	return { x: 48 * direction, duration: 320, easing: cubicOut };
}
