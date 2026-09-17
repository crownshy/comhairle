import { expect, type Locator } from '@playwright/test';
import { Page } from '../types';
import UserInputs, { DerivedUserInputsReturn, UserInputsInput } from './UserInputs';

type Textbox<T extends string> = {
	id: T;
	locator: Locator;
	value: string;
};

function Textboxes<const T extends string, U extends Textbox<T>>(
	page: Page,
	inputs: UserInputsInput<T>,
	cleanupRef?: (callback: () => Promise<void>) => void
): DerivedUserInputsReturn<T, U> {
	const textboxes = UserInputs<T, U>(
		inputs,
		(name) =>
			({
				locator: page.getByRole('textbox', { name, exact: true })
			}) as U
	);

	return {
		get(id) {
			return textboxes.get(id);
		},
		write(id, resetValue) {
			return textboxes.write(
				id,
				async (textbox) => {
					await textbox.locator.click();
					await textbox.locator.fill(textbox.value);
				},
				(textbox) => {
					if (resetValue) {
						cleanupRef?.(async () => {
							await textbox.locator.click();
							await textbox.locator.fill(resetValue ?? '');
						});
					}
				}
			);
		},
		expect() {
			return textboxes.expect((textbox) =>
				expect(textbox.locator).toHaveValue(textbox.value)
			);
		}
	};
}

export default Textboxes;
