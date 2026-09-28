import { exists } from '..';
import UserInputs, { Refs, UserInputsInput } from './UserInputs';

type CollapsibleRichField<T> = {
	id: T;
	name: string;
	fallbackIndex: number;
	value: string;
};

const CollapisbleRichFields = <const T extends string, U extends CollapsibleRichField<T>>(
	inputs: UserInputsInput<T>,
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
			await refs.page.locator('.tiptap.ProseMirror').click();
			await refs.page.locator('.tiptap.ProseMirror').fill(value);
			await refs.page.getByRole('button', { name: 'Done' }).click();
		},
		async expector(collapisbleRichField) {
			await exists(refs.page.getByRole('button', { name: collapisbleRichField.value }));
		}
	});

export default CollapisbleRichFields;
