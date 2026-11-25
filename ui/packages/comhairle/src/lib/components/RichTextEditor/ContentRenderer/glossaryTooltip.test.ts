// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { attachGlossaryTooltip } from './glossaryTooltip';

let detach: () => void;
let container: HTMLElement;
let tooltip: HTMLElement;
let term: HTMLButtonElement;

const isVisible = () => tooltip.getAttribute('data-visible') === 'true';

beforeEach(() => {
	vi.useFakeTimers();
	document.body.innerHTML = `
		<div id="content">
			<p>Take the <button type="button" class="glossary-term" data-glossary-tooltip="A vehicle">bus</button> home</p>
		</div>
		<div id="glossary-tooltip" role="tooltip" aria-hidden="true"></div>
		<button id="elsewhere" type="button">Elsewhere</button>
	`;
	container = document.getElementById('content')!;
	tooltip = document.getElementById('glossary-tooltip')!;
	term = container.querySelector('.glossary-term')!;
	detach = attachGlossaryTooltip(container, tooltip);
});

afterEach(() => {
	detach();
	vi.useRealTimers();
	document.body.replaceChildren();
});

function mouseOut(from: Element, to: Element | null) {
	from.dispatchEvent(new MouseEvent('mouseout', { bubbles: true, relatedTarget: to }));
}

describe('attachGlossaryTooltip', () => {
	it('opens on focus and describes the term', () => {
		term.focus();
		expect(isVisible()).toBe(true);
		expect(tooltip.textContent).toBe('A vehicle');
		expect(term.getAttribute('aria-describedby')).toBe('glossary-tooltip');
	});

	it('closes on Escape without moving focus', () => {
		term.focus();
		window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }));
		expect(isVisible()).toBe(false);
		expect(term.hasAttribute('aria-describedby')).toBe(false);
		expect(document.activeElement).toBe(term);
	});

	it('closes when focus leaves the term', () => {
		term.focus();
		document.getElementById('elsewhere')!.focus();
		expect(isVisible()).toBe(false);
	});

	it('stays open while the pointer moves from the term onto the tooltip', () => {
		term.dispatchEvent(new MouseEvent('mouseover', { bubbles: true }));
		mouseOut(term, tooltip);
		tooltip.dispatchEvent(new MouseEvent('mouseenter'));
		vi.runAllTimers();
		expect(isVisible()).toBe(true);
	});

	it('stays open when the pointer crosses the gap to the tooltip', () => {
		term.dispatchEvent(new MouseEvent('mouseover', { bubbles: true }));
		mouseOut(term, container);
		tooltip.dispatchEvent(new MouseEvent('mouseenter'));
		vi.runAllTimers();
		expect(isVisible()).toBe(true);
	});

	it('closes shortly after the pointer leaves both the term and the tooltip', () => {
		term.dispatchEvent(new MouseEvent('mouseover', { bubbles: true }));
		mouseOut(term, container);
		expect(isVisible()).toBe(true);
		vi.runAllTimers();
		expect(isVisible()).toBe(false);
	});

	it('stays open on pointer leave while the term still has focus', () => {
		term.focus();
		mouseOut(term, container);
		vi.runAllTimers();
		expect(isVisible()).toBe(true);
	});
});
