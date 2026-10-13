import { expect } from '@playwright/test';
import type { Page } from '../../types';
import type { Refs } from '../types';
import { generateValue } from '../..';

// Rich text editor "Tiptap"

const getTiptap = async (page: Page, index: number = 0) => {
	const locator = page.locator('.tiptap.ProseMirror').nth(index);
	if ((await locator.count()) > 0) {
		return locator;
	}
	return page.locator('.tiptap').nth(index);
};

const Tiptap = (refs: Refs) => {
	const values: string[] = [];

	return {
		write: async (text: string = generateValue(), index: number = 0): Promise<void> => {
			const tiptap = await getTiptap(refs.page, index);
			values[index] = text;
			await tiptap.click();
			await tiptap.fill(text);

			refs.cleanup(async () => {
				await tiptap.click();
				await tiptap.fill('');
			});
		},
		expect: async () => {
			for (let i = 0; i < values.length; i++) {
				const value = values[i];
				if (value === undefined) {
					continue;
				}
				await expect(await getTiptap(refs.page, i)).toContainText(value);
			}
		}
	} as const;
};

export default Tiptap;
