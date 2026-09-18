import { expect, type Locator } from '@playwright/test';
import UserInputs, { Refs, UserInputsInput } from './UserInputs';

type Textbox<T extends string> = {
	id: T;
	locator: Locator;
	value: string;
};

const Textboxes = <const T extends string, U extends Textbox<T>>(
	inputs: UserInputsInput<T>,
	refs: Refs
) =>
	UserInputs<T, U>({
		inputs,
		mutator: (name) =>
			({
				locator: refs.page.getByRole('textbox', { name, exact: true })
			}) as U,
		cleanup: refs.cleanup,
		async focus(textbox) {
			await textbox.locator.click();
		},
		async update(textbox, value) {
			await textbox.locator.fill(value);
		},
		async expector(textbox) {
			await expect(textbox.locator).toHaveValue(textbox.value);
		}
	});

export default Textboxes;
