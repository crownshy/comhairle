import { expect, type Locator } from '@playwright/test';
import { Page } from '../types';
import UserInputs, { UserInputsInput } from './UserInputs';
import type { Cleanup } from './types';

type Textbox<T extends string> = {
	id: T;
	locator: Locator;
	value: string;
};

const Textboxes = <const T extends string, U extends Textbox<T>>(
	page: Page,
	inputs: UserInputsInput<T>,
	cleanup: Cleanup
) =>
	UserInputs<T, U>({
		inputs,
		mutator: (name) =>
			({
				locator: page.getByRole('textbox', { name, exact: true })
			}) as U,
		cleanup,
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
