import type { Parent } from '$lib/tools/heyform/utils';
import type { Attachment } from 'svelte/attachments';

function openNest(element: Element, title: string) {
	element.outerHTML =
		`<section class='border'><span class='font-bold text-xl'>${title}</span>` +
		element.outerHTML;
}
function closeNest(element: Element) {
	element.outerHTML = '</section>' + element.outerHTML;
}

export function handleNested(
	previousParent: Parent | undefined,
	parent: Parent | undefined
): Attachment {
	let attachment: Attachment = () => {};
	console.log('previousParent:', previousParent);
	console.log('parent:', parent);
	if (previousParent?.id !== undefined && parent?.id === undefined) {
		// If the previous quesiton did have a parent, but this one doesn't then close the nesting
		attachment = (element) => {
			// closeNest(element);
		};
	} else if (previousParent?.id === undefined && parent?.id !== undefined) {
		// If the previous quesiton didn't have a parent, but this one does then open the nesting
		attachment = (element) => {
			openNest(element, parent.title);
		};
	} else if (
		previousParent?.id !== undefined &&
		parent?.id !== undefined &&
		previousParent.id !== parent.id
	) {
		// If the parent ids don't match then it's a new nested question, so close the old one and open a new one
		attachment = (element) => {
			closeNest(element);
			openNest(element, parent.title);
		};
	}
	return attachment;
}
