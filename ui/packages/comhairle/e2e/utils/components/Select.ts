import { expect } from '@playwright/test';
import type { Refs } from './types';

function Select<const T extends string[]>(inputs: T, refs: Refs) {
	const originallySelected: T[number] = inputs[0];
	let selected: T[number] = inputs[0];

	async function getSelect() {
		const locator = refs.page.getByRole('button', { name: selected });
		if ((await locator.count()) === 0) {
			throw new Error(`Could not find locator: ${selected}`);
		}
		return locator;
	}

	async function getMenuItem(option: T[number]) {
		const locator = refs.page.getByRole('menuitem', { name: option });
		if ((await locator.count()) === 0) {
			throw new Error(`Could not find locator: ${option}`);
		}
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

	refs.cleanup(async () => {
		await pick(originallySelected);
	});

	return {
		pick,
		expect: async () => expect(await getSelect()).toContainText(selected)
	};
}

export default Select;
