import { expect } from '@playwright/test';
import UserInput from './UserInput';
import type { Refs } from '../types';
import { find } from '../..';

type Options = {
	resetTo?: string;
	index?: number;
	overrideRole?: 'spinbutton'; // For numeric inputs
	exact?: boolean; // Exact match for name
};

const Textbox = async (name: string | undefined, refs: Refs, options: Options = {}) => {
	const locator = () =>
		find(
			refs.page
				.getByRole(options?.overrideRole ?? 'textbox', { name, exact: options.exact })
				.nth(options.index ?? 0)
		);

	const userInputs = await UserInput({
		focus: async () => (await locator()).click(),
		update: async (value) => (await locator()).fill(value),
		expector: async (currentValue) => expect(await locator()).toHaveValue(currentValue)
	});

	refs.cleanup?.(() => userInputs.write(options.resetTo ?? ''));

	return {
		...userInputs,
		locator,
		isValid: async () =>
			(await locator()).evaluate((element) => (element as HTMLInputElement).validity.valid)
	};
};

export default Textbox;
