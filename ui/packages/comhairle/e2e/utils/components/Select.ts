import { expect } from '@playwright/test';
import type { Refs } from './types';
import { find } from '..';
import Button from './Button';

function Select<const T extends string[]>(inputs: T, refs: Refs) {
	const originallySelected: T[number] = inputs[0];
	let selected: T[number] = inputs[0];

	const getSelect = () => Button(selected, refs.page);

	async function getMenuItem(option: T[number]) {
		const locator = await find(
			refs.page
				.getByRole('menuitem', { name: option })
				.or(refs.page.getByRole('option', { name: option }))
		);
		return locator;
	}

	async function open() {
		await (await getSelect()).click();
	}

	async function pick(option: T[number]) {
		await open();
		await (await getMenuItem(option)).click();
		selected = option;
	}

	refs.cleanup?.(async () => {
		await pick(originallySelected);
	});

	return {
		pick,
		expect: async () => expect(await getSelect()).toContainText(selected)
	};
}

export default Select;
