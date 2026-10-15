import { expect } from '@playwright/test';
import type { Refs } from './types';
import { find } from '..';

async function RadioButtons<const T extends string[]>(options: T, refs: Refs) {
	const getRadio = (name: string) => find(refs.page.getByRole('radio', { name }));

	let selected: T[number] | undefined = await (async () => {
		for (const option of options) {
			if (await (await getRadio(option)).isChecked()) return option;
		}
		return undefined;
	})();

	const firstSelected = selected;

	async function select(option: T[number]) {
		await (await getRadio(option)).click();
		selected = option;
	}

	async function expected() {
		if (!selected) {
			return;
		}
		expect(await (await getRadio(selected)).isChecked()).toBe(true);
	}

	refs.cleanup?.(async () => {
		if (!firstSelected) {
			return;
		}
		await (await getRadio(firstSelected)).click();
	});

	return {
		select,
		expect: expected,
		value: () => selected
	};
}

export default RadioButtons;
