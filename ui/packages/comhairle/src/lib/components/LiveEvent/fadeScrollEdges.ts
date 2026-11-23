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
	const observeChildren = () => {
		for (const child of element.children) resizeObserver.observe(child);
	};
	resizeObserver.observe(element);
	observeChildren();

	// The question card can mount after the container (it sits in an {#if}), so watch for new children.
	const mutationObserver = new MutationObserver(() => {
		observeChildren();
		update();
	});
	mutationObserver.observe(element, { childList: true });

	element.addEventListener('scroll', update, { passive: true });
	update();

	return () => {
		element.removeEventListener('scroll', update);
		resizeObserver.disconnect();
		mutationObserver.disconnect();
	};
};
