import { expect, type Locator } from '@playwright/test';
import UserInput from './UserInput';
import type { Refs } from '../types';

type Options = {
	resetTo?: string;
	index?: number;
	overrideRole?: 'spinbutton'; // For numeric inputs
};

const Textbox = async (name: string | undefined, refs: Refs, options: Options = {}) => {
	const locator = () =>
		refs.page.getByRole(options?.overrideRole ?? 'textbox', { name }).nth(options.index ?? 0);

	const initialValue = await locator().inputValue();

	const userInputs = await UserInput({
		focus: () => locator().click(),
		update: (value) => locator().fill(value),
		expector: (currentValue) => expect(locator()).toHaveValue(currentValue)
	});

	refs.cleanup?.(() => userInputs.write(options.resetTo ?? initialValue));

	return {
		...userInputs,
		locator,
		isValid: () => locator().evaluate((element) => (element as HTMLInputElement).validity.valid)
	};
};

export default Textbox;
