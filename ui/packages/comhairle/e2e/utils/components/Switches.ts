import { expect, type Locator } from '@playwright/test';
import type { Locators, Refs } from './types';

type Switches<T extends string> = {
	id: T;
	locator: Locator;
	startValue: boolean;
	endValue: boolean;
};

const isSwitchOn = async (_switch: Locator): Promise<boolean> =>
	_switch.getAttribute('data-state').then((state) => state === 'checked');

const Switches = async <const T extends string, U extends Switches<T>>(
	inputs: Locators<T>,
	refs: Refs
) => {
	const switches: Record<T, U> = Object.fromEntries(
		await Promise.all(
			inputs.map(async (input) => {
				const locator = refs.page.getByRole('switch', { name: input[1] });
				if (!locator.isVisible()) {
					throw new Error(`Can't find locator: "${input[1]}"`);
				}
				const isOn = await isSwitchOn(locator);
				return [
					input[0],
					{
						id: input[0],
						locator,
						startValue: isOn,
						endValue: !isOn
					} as U
				] as const;
			})
		)
	) as Record<T, U>;

	async function set(id: T, shouldBeOn: boolean) {
		const _switch = switches[id];
		const isOn = await isSwitchOn(_switch.locator);
		if ((isOn && shouldBeOn) || (!isOn && !shouldBeOn)) {
			return;
		}
		await _switch.locator.click();
	}

	async function toggle(id: T) {
		await set(id, switches[id].endValue);
		refs.cleanup(async () => {
			await set(id, switches[id].startValue);
		});
	}

	async function expector() {
		for (const _switch of Object.values(switches) as U[]) {
			const isOn = await isSwitchOn(_switch.locator);
			expect(_switch.endValue).toBe(isOn);
		}
	}

	return {
		toggle,
		expect: expector
	} as const;
};

export default Switches;
