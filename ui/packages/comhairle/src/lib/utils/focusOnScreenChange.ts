import type { Attachment } from 'svelte/attachments';

/**
 * Focuses the element when it mounts, so a screen that replaces the last one is announced
 * and keyboard users carry on from it instead of from the top of the page. Put it on the
 * new screen's heading and give that `tabindex="-1"`.
 *
 * It only acts when focus has dropped to the page body, which is what happens when the
 * control the participant used went away with the old screen. If focus is still on
 * something that stayed put, the participant is busy there and keeps their place.
 */
export const focusOnScreenChange: Attachment<HTMLElement> = (element) => {
	const active = element.ownerDocument.activeElement;
	if (active === null || active === element.ownerDocument.body) element.focus();
};
