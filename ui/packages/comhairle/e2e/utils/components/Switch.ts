import { expect } from '@playwright/test';
import type { Refs } from './types';
import { eventually } from '../testing';
import { find } from '..';

async function Switch(name: string, refs: Refs) {
	const locator = () => find(refs.page.getByRole('switch', { name }));

	const isSwitchOn = async (): Promise<boolean> =>
		(await locator()).getAttribute('data-state').then((state) => state === 'checked');

	const startsOn = await (async () => {
		await eventually(async () => {
			await (await locator()).isVisible();
		});
		return isSwitchOn();
	})();

	let isOn = startsOn;

	const toggle = async () => {
		await (await locator()).click();
		isOn = !isOn;
	};

	async function set(shouldBeOn: boolean) {
		const isOn = await isSwitchOn();
		if ((isOn && shouldBeOn) || (!isOn && !shouldBeOn)) {
			return;
		}
		await toggle();
	}

	const expector = async () => expect(isOn).toBe(await isSwitchOn());

	refs.cleanup?.(async () => {
		await set(startsOn);
	});

	return {
		toggle,
		expect: expector
	} as const;
}

export default Switch;
