import { exists } from '../..';
import UserInputs from './UserInputs';
import type { Locators, Refs } from '../types';
import Tiptap from './Tiptap';

type CollapsibleRichFields<T> = {
	id: T;
	name: string;
	fallbackIndex: number;
	value: string;
};

const CollapsibleRichFields = <const T extends string, U extends CollapsibleRichFields<T>>(
	inputs: Locators<T>,
	refs: Refs
) =>
	UserInputs<T, U>({
		inputs,
		mutator: (name, index) =>
			({
				name,
				fallbackIndex: index
			}) as U,
		cleanup: refs.cleanup,
		async focus(collapisbleRichField) {
			const btn = refs.page.getByRole('button', { name: collapisbleRichField.name });
			if (await exists(btn)) {
				btn.click();
				return;
			}
			const editWithValue = refs.page.getByRole('button', {
				name: collapisbleRichField.value
			});
			if (await exists(editWithValue)) {
				editWithValue.click();
				return;
			}
			await refs.page
				.getByRole('button', { name: 'Edit' })
				.nth(collapisbleRichField.fallbackIndex)
				.click();
		},
		async update(_, value) {
			const tiptap = Tiptap(refs);
			await tiptap.write(value);
			await refs.page.getByRole('button', { name: 'Done' }).click();
		},
		async expector(collapisbleRichField) {
			await exists(refs.page.getByRole('button', { name: collapisbleRichField.value }));
		}
	});

export default CollapsibleRichFields;
