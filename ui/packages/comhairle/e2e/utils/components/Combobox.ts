import type { Refs } from './types';
import { eventually } from '../testing';
import { expect } from '@playwright/test';
import { find } from '..';

function Combobox(initialName: string, refs: Refs) {
	let name = initialName;

	async function open() {
		await (await find(refs.page.getByText(name))).click();
	}

	async function search(searchTerm: string) {
		const searchbar = refs.page.getByPlaceholder(name);
		await searchbar.click();
		await searchbar.fill(searchTerm);
	}

	async function select(option: string) {
		await open();

		await (await find(refs.page.getByRole('option', { name }))).click();

		// Wait for dialog to close
		await eventually(() =>
			expect(refs.page.getByRole('listbox', { name: 'Suggesstions...' })).not.toBeVisible()
		);

		name = option;
	}

	return {
		open,
		search,
		select,
		value: () => name
	};
}

export default Combobox;
