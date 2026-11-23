import type { Attachment } from 'svelte/attachments';
import { scrollFadeMask } from '$lib/utils/scrollFade';

/**
 * Fades the edges of a scroll container that have more content past them. Re-measures on
 * scroll and whenever the container or its content changes size (text zoom, a new question).
 */
export const fadeScrollEdges: Attachment<HTMLElement> = (element) => {
	const update = () => {
		element.style.maskImage = scrollFadeMask(element);
	};

	const resizeObserver = new ResizeObserver(update);
	resizeObserver.observe(element);
	for (const child of element.children) resizeObserver.observe(child);

	element.addEventListener('scroll', update, { passive: true });
	update();

	return () => {
		element.removeEventListener('scroll', update);
		resizeObserver.disconnect();
	};
};
