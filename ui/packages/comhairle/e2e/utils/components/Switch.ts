import { expect } from '@playwright/test';
import type { Refs } from './types';

const dataState = (isOn: boolean) => (isOn ? 'checked' : 'unchecked');

async function Switch(name: string, refs: Refs) {
	const locator = () => refs.page.getByRole('switch', { name });

	const isSwitchOn = (): Promise<boolean> =>
		locator()
			.getAttribute('data-state')
			.then((state) => state === 'checked');

	await expect(locator(), `Can't find switch: "${name}"`).toBeVisible();
	const startsOn = await isSwitchOn();

	let isOn = startsOn;

	const toggle = async () => {
		await locator().click();
		isOn = !isOn;
	};

	async function set(shouldBeOn: boolean) {
		if ((await isSwitchOn()) === shouldBeOn) {
			return;
		}
		await toggle();
	}

	const expector = () => expect(locator()).toHaveAttribute('data-state', dataState(isOn));

	refs.cleanup?.(async () => {
		await set(startsOn);
	});

	return {
		toggle,
		expect: expector
	} as const;
}

export default Switch;
