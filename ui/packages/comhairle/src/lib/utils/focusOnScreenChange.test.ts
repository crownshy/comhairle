// @vitest-environment jsdom
import { afterEach, describe, expect, it } from 'vitest';
import { focusOnScreenChange } from './focusOnScreenChange';

function mountHeading(): HTMLElement {
	const heading = document.createElement('h2');
	heading.tabIndex = -1;
	document.body.append(heading);
	return heading;
}

afterEach(() => {
	document.body.replaceChildren();
});

describe('focusOnScreenChange', () => {
	it('focuses the heading when the control the participant used has gone', () => {
		const button = document.createElement('button');
		document.body.append(button);
		button.focus();
		button.remove();

		const heading = mountHeading();
		focusOnScreenChange(heading);

		expect(document.activeElement).toBe(heading);
	});

	it('leaves focus on a control that is still on the page', () => {
		const button = document.createElement('button');
		document.body.append(button);
		button.focus();

		const heading = mountHeading();
		focusOnScreenChange(heading);

		expect(document.activeElement).toBe(button);
	});
});
