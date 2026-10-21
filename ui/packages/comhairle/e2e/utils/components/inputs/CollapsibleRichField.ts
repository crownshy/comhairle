import { expect } from '@playwright/test';
import UserInput from './UserInput';
import type { Refs } from '../types';

type Options = {
	resetTo?: string;
};

/**
 * A rich text field that sits collapsed behind an "Add <label>" or "Edit <label>" button until
 * opened. Only one is open at a time, so the open editor is the only one on the page.
 */
const CollapsibleRichField = async (label: string, refs: Refs, options: Options = {}) => {
	const collapsed = () =>
		refs.page.getByRole('button', { name: new RegExp(`^(Add|Edit) ${label}$`, 'i') });

	const userInput = await UserInput({
		focus: () => collapsed().click(),
		async update(value) {
			const editor = refs.page.locator('.tiptap.ProseMirror');
			await editor.click();
			await editor.fill(value);
			await refs.page.getByRole('button', { name: 'Done' }).click();
		},
		expector: (currentValue) => expect(collapsed()).toContainText(currentValue)
	});

	refs.cleanup?.(async () => {
		await userInput.write(options.resetTo ?? ' ');
	});

	return userInput;
};

export default CollapsibleRichField;
