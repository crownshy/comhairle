import { expect, type Locator } from '@playwright/test';
import UserInputs, { Refs, UserInputsInput } from './UserInputs';

type Textbox<T extends string> = {
	id: T;
	locator: Locator;
	value: string;
};

const IsValid = (locator: Locator): Promise<boolean> =>
	locator.evaluate((element) => (element as HTMLInputElement).validity.valid);

const New = <const T extends string, U extends Textbox<T>>(
	inputs: UserInputsInput<T>,
	refs: Refs
) => {
	const userInputs = UserInputs<T, U>({
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

	return {
		...userInputs,
		isValid: (id: T): Promise<boolean> => IsValid(userInputs.get(id).locator)
	};
};

const Textboxes = {
	new: New,
	isValid: IsValid
};

export default Textboxes;
