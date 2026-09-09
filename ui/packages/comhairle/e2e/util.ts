import type { Locator, PlaywrightTestArgs } from '@playwright/test';
import { m } from '../src/lib/paraglide/messages';
import { TEST_CONVERSATION } from './constants';

type Page = PlaywrightTestArgs['page'];

export const exists = async (locator: Locator): Promise<boolean> => !!(await locator.count());

export async function gotoComhairle(page: Page) {
	await page.goto('http://localhost:4173/');
}

export async function login(page: Page) {
	const workspace = page.getByRole('link', { name: m.workspace() });

	// Already logged in
	if (await exists(workspace)) {
		await workspace.click();
		return;
	}

	// Login
	await page.getByRole('link', { name: m.login() }).click();

	const email = page.getByRole('textbox', { name: m.email() });
	await email.click();
	await email.fill('admin@crown-shy.com');
	await email.press('Tab');

	await page.getByRole('textbox', { name: 'Password' }).fill('adminPassword123!');
	await page.getByRole('button', { name: 'Log In' }).click();
}

export async function openConversation(page: Page) {
	const conversation = page.getByRole('link', { name: TEST_CONVERSATION });

	// Conversation already exists
	if (await exists(conversation)) {
		conversation.click();
		return;
	}

	// Make conversation
	await page.getByText('Start from blank Choose from').click();
	await page.getByRole('menuitem', { name: 'Start from blank' }).click();
	await page.getByRole('textbox', { name: 'Title' }).fill(TEST_CONVERSATION);
}
