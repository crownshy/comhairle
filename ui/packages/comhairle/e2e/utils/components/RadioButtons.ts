import { expect } from '@playwright/test';
import type { Refs } from './types';
import { find } from '..';

type Options<T extends string[]> = {
	resetTo?: T[number];
};

async function RadioButtons<const T extends string[]>(radios: T, refs: Refs, options?: Options<T>) {
	const getRadio = (name: string) => find(refs.page.getByRole('radio', { name }));

	let selected: T[number] | undefined = await (async () => {
		for (const radio of radios) {
			if (await (await getRadio(radio)).isChecked()) return radio;
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
		if (options?.resetTo) {
			await (await getRadio(options.resetTo)).click();
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
