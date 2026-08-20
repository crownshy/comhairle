import type { Parent } from '$lib/tools/heyform/utils';
import type { Attachment } from 'svelte/attachments';

function getOrInsertParent(parent: Parent, element: Element): Element {
	const parentElement = document.getElementById(parent.id);
	if (parentElement !== null) {
		return parentElement;
	}

	// Nested container
	const section = document.createElement('section');
	section.id = parent.id;
	section.className = 'border rounded p-6';

	// Title
	const title = document.createElement('span');
	title.className = 'font-bold text-xl';
	title.textContent = parent.title;
	section.appendChild(title);

	element.before(section);
	return section;
}

export function handleNested(parent: Parent | undefined): Attachment {
	return (element) => {
		if (parent === undefined) {
			return;
		}
		// Get nested section
		const parentElement = getOrInsertParent(parent, element);

		// Indent all nested questions
		element.className += ' pl-5';

		// Move element to inside of the nested section
		parentElement.appendChild(element);
	};
}
