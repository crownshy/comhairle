import { expect, type Locator } from '@playwright/test';
import UserInput from './UserInput';
import type { Refs } from '../types';

type Options = {
	resetTo?: string;
	index?: number;
};

const Textbox = async (name: string, refs: Refs, options: Options = {}) => {
	const locator = () => refs.page.getByRole('textbox', { name }).nth(options.index ?? 0);

	const initialValue = await locator().inputValue();

	const userInputs = await UserInput({
		focus: () => locator().click(),
		update: (value) => locator().fill(value),
		expector: (currentValue) => expect(locator()).toHaveValue(currentValue)
	});

	refs.cleanup(() => userInputs.write(options.resetTo ?? initialValue));

	return {
		...userInputs,
		locator,
		isValid: (locator: Locator) =>
			locator.evaluate((element) => (element as HTMLInputElement).validity.valid)
	};
};

export default Textbox;
