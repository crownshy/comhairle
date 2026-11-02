import { expect } from '@playwright/test';
import type { Page } from '../../types';
import type { Refs } from '../types';
import { generateValue, find } from '../..';

// Rich text editor "Tiptap"
async function getTiptap(page: Page, index: number = 0) {
	const locator = find(page.locator('.tiptap.ProseMirror').nth(index));
	if ((await (await locator).count()) > 0) {
		return locator;
	}
	return page.locator('.tiptap').nth(index);
}

function Tiptap(index: number, refs: Refs) {
	let value: string | undefined;

	refs.cleanup?.(async () => {
		const tiptap = await getTiptap(refs.page, index);
		await tiptap.click();
		await tiptap.fill('');
	});

	return {
		write: async (text = generateValue()): Promise<void> => {
			const tiptap = await getTiptap(refs.page, index);
			value = text;
			await tiptap.click();
			await tiptap.fill(text);
		},
		expect: async () => {
			if (value === undefined) return;
			await expect(await getTiptap(refs.page, index)).toContainText(value);
		}
	} as const;
}

export default Tiptap;
