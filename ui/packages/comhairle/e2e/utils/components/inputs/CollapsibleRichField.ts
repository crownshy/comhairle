import { exists, sleep, find } from '../..';
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
		async focus() {
			const btn = await find(refs.page.getByRole('button', { name }));
			if (await exists(btn)) {
				await btn.click();
				return;
			}
			await refs.page.getByRole('button', { name: 'Edit' }).nth(fallbackIndex).click();
		},
		async update(value) {
			// Wait for tiptap to load
			await sleep(0.2);
			const tiptap = Tiptap(fallbackIndex, { page: refs.page });
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
