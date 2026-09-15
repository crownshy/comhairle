import { expect } from '@playwright/test';
import { test } from './utils/testing';
import { exists } from './utils';
import Conversation from './utils/navigation/Conversation';
import type { Page } from './utils/types';
import Textboxes from './utils/inputs/Textboxes';
import { login } from './utils/auth';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Content');
});

async function edit(page: Page, name: string, fallbackIndex: number) {
	const btn = page.getByRole('button', { name });
	if (await exists(btn)) {
		btn.click();
		return;
	}
	await page.getByRole('button', { name: 'Edit' }).nth(fallbackIndex).click();
}

async function write(page: Page, index: number, text: string) {
	await page.locator('.tiptap').nth(index).click();
	await page.locator('.tiptap').nth(index).fill(text);
	await page.getByRole('button', { name: 'Done' }).click();
}

test('Configure/Content page', async ({ page, cleanup }) => {
	// Setup
	const NAMES = [
		'Add privacy policy',
		'Add short privacy policy',
		'Add faqs',
		'Add thank you message'
	] as const;

	const NEW_VALUES: string[] = [];

	for (let i = 0; i < NAMES.length; i++) {
		NEW_VALUES.push(crypto.randomUUID());
	}

	let i = 0;

	for (const name of NAMES) {
		await edit(page, name, i);
		await write(page, i, NEW_VALUES[i]);
		cleanup(async () => {
			await edit(page, name, i);
			await write(page, i, '');
		});
		i++;
	}

	const textboxes = new Textboxes(page, [['cta', 'Call to action']]);
	textboxes.write('cta', cleanup, '');

	// Checks
	for (const newValue of NEW_VALUES) {
		expect(await exists(page.getByRole('button', { name: newValue }))).toBe(true);
	}

	await textboxes.expected();
});
