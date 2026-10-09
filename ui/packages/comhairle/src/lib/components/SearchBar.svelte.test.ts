// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { flushSync, mount, unmount } from 'svelte';
import SearchBar from './SearchBar.svelte';

let component: ReturnType<typeof mount> | undefined;

afterEach(async () => {
	if (component) await unmount(component);
	component = undefined;
	document.body.replaceChildren();
});

describe('SearchBar', () => {
	it('renders a native search input and forwards input attributes', () => {
		component = mount(SearchBar, {
			target: document.body,
			props: {
				value: 'Dublin',
				id: 'area-search',
				name: 'area',
				placeholder: 'Search areas',
				'aria-label': 'Search geographic areas',
				disabled: true,
				class: 'h-12'
			}
		});
		flushSync();

		const input = document.querySelector('input')!;
		expect(input.type).toBe('search');
		expect(input.value).toBe('Dublin');
		expect(input.id).toBe('area-search');
		expect(input.name).toBe('area');
		expect(input.placeholder).toBe('Search areas');
		expect(input.getAttribute('aria-label')).toBe('Search geographic areas');
		expect(input.disabled).toBe(true);
		expect(input.classList.contains('h-12')).toBe(true);
		expect(input.classList.contains('pl-9')).toBe(true);
		expect(input.parentElement?.classList.contains('pile')).toBe(true);
		expect(document.querySelector('svg')?.getAttribute('aria-hidden')).toBe('true');
		expect(document.querySelector('svg')?.classList.contains('pointer-events-none')).toBe(true);
	});

	it('updates the bound value before forwarding typing and clearing events', () => {
		let value = '';
		const oninput = vi.fn(() => value);
		component = mount(SearchBar, {
			target: document.body,
			props: {
				get value() {
					return value;
				},
				set value(nextValue: string) {
					value = nextValue;
				},
				oninput,
				'aria-label': 'Search geographic areas'
			}
		});
		flushSync();

		const input = document.querySelector('input')!;
		for (const nextValue of ['Dublin', '']) {
			input.value = nextValue;
			input.dispatchEvent(new Event('input', { bubbles: true }));
			flushSync();

			expect(value).toBe(nextValue);
			expect(oninput).toHaveLastReturnedWith(nextValue);
		}
		expect(oninput).toHaveBeenCalledTimes(2);
	});
});
