import type { Page } from '../types';
import { exists } from '..';
import UserInputs, { DerivedUserInputsReturn, UserInputsInput } from './UserInputs';

type CollapsibleRichField<T> = {
	id: T;
	name: string;
	fallbackIndex: number;
	value: string;
};

function CollapisbleRichFields<const T extends string, U extends CollapsibleRichField<T>>(
	page: Page,
	inputs: UserInputsInput<T>,
	cleanupRef?: (callback: () => Promise<void>) => void
): DerivedUserInputsReturn<T, U> {
	const collapisbleRichFields = UserInputs<T, U>(
		inputs,
		(name, index) =>
			({
				name,
				fallbackIndex: index
			}) as U
	);

	async function edit(id: T) {
		const btn = page.getByRole('button', { name: collapisbleRichFields.get(id).name });
		if (await exists(btn)) {
			btn.click();
			return;
		}
		const editWithValue = page.getByRole('button', {
			name: collapisbleRichFields.get(id).value
		});
		if (await exists(editWithValue)) {
			editWithValue.click();
			return;
		}
		await page
			.getByRole('button', { name: 'Edit' })
			.nth(collapisbleRichFields.get(id).fallbackIndex)
			.click();
	}

	async function write(index: number, text: string) {
		await page.locator('.tiptap').nth(index).click();
		await page.locator('.tiptap').nth(index).fill(text);
		await page.getByRole('button', { name: 'Done' }).click();
	}

	return {
		get(id) {
			return collapisbleRichFields.get(id);
		},
		async write(id, resetValue) {
			return collapisbleRichFields.write(
				id,
				async (collapisbleRichField) => {
					await edit(collapisbleRichField.id);
					await write(collapisbleRichField.fallbackIndex, collapisbleRichField.value);
				},
				(collapisbleRichField) => {
					if (resetValue) {
						cleanupRef?.(async () => {
							await edit(collapisbleRichField.id);
							await write(collapisbleRichField.fallbackIndex, resetValue ?? '');
						});
					}
				}
			);
		},
		async expect() {
			await collapisbleRichFields.expect(async (collapisbleRichField) => {
				await exists(page.getByRole('button', { name: collapisbleRichField.value }));
			});
		}
	};
}

export default CollapisbleRichFields;
