import type { Page } from '../types';
import { exists } from '..';
import UserInputs, { UserInputsInput } from './UserInputs';

type CollapsibleRichField<T> = {
	id: T;
	name: string;
	fallbackIndex: number;
	value: string;
};

const CollapisbleRichFields = <const T extends string, U extends CollapsibleRichField<T>>(
	page: Page,
	inputs: UserInputsInput<T>,
	cleanup: (callback: () => Promise<void>) => void
) =>
	UserInputs<T, U>({
		inputs,
		mutator: (name, index) =>
			({
				name,
				fallbackIndex: index
			}) as U,
		cleanup,
		async focus(collapisbleRichField) {
			const btn = page.getByRole('button', { name: collapisbleRichField.name });
			if (await exists(btn)) {
				btn.click();
				return;
			}
			const editWithValue = page.getByRole('button', {
				name: collapisbleRichField.value
			});
			if (await exists(editWithValue)) {
				editWithValue.click();
				return;
			}
			await page
				.getByRole('button', { name: 'Edit' })
				.nth(collapisbleRichField.fallbackIndex)
				.click();
		},
		async update(collapisbleRichField, value) {
			await page.locator('.tiptap').nth(collapisbleRichField.fallbackIndex).click();
			await page.locator('.tiptap').nth(collapisbleRichField.fallbackIndex).fill(value);
			await page.getByRole('button', { name: 'Done' }).click();
		},
		async expector(collapisbleRichField) {
			await exists(page.getByRole('button', { name: collapisbleRichField.value }));
		}
	});

export default CollapisbleRichFields;
