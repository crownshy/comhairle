import { exists } from '../..';
import UserInput from './UserInput';
import type { Refs } from '../types';
import Tiptap from './Tiptap';

type Options = {
	resetTo?: string;
};

const CollapsibleRichField = async (
	name: string,
	fallbackIndex: number,
	refs: Refs,
	options: Options = {}
) => {
	const userInput = await UserInput({
		async focus(currentValue) {
			const btn = refs.page.getByRole('button', { name });
			if (await exists(btn)) {
				btn.click();
				return;
			}
			const EditWithValue = refs.page.getByRole('button', {
				name: currentValue
			});
			if (await exists(EditWithValue)) {
				EditWithValue.click();
				return;
			}
			await refs.page.getByRole('button', { name: 'Edit' }).nth(fallbackIndex).click();
		},
		async update(value) {
			const tiptap = Tiptap(fallbackIndex, refs);
			await tiptap.write(value);
			await refs.page.getByRole('button', { name: 'Done' }).click();
		},
		expector: (currentValue) => exists(refs.page.getByRole('button', { name: currentValue }))
	});

	refs.cleanup?.(async () => {
		await userInput.write(options?.resetTo ?? ' ');
	});

	return userInput;
};

export default CollapsibleRichField;
