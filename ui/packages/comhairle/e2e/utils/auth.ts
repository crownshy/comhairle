import { m } from '../../src/lib/paraglide/messages';
import type { Page } from './types';

// Every test gets a fresh browser context, so there is never a session to reuse.
export async function login(page: Page) {
	const username = process.env.PLAYWRIGHT_ADMIN_USERNAME;
	const password = process.env.PLAYWRIGHT_ADMIN_PASSWORD;
	if (username === undefined || password === undefined) {
		throw new Error(
			'Missing PLAYWRIGHT_ADMIN_USERNAME or PLAYWRIGHT_ADMIN_PASSWORD in the .playwright.env file'
		);
	}

	await page.goto('/auth/login');
	await page.getByRole('textbox', { name: m.email() }).fill(username);
	await page.getByRole('textbox', { name: 'Password' }).fill(password);
	await page.getByRole('button', { name: m.login() }).click();

	// /admin redirects to the conversations list for now, so accept either.
	await page.waitForURL(/\/admin(\/conversations)?$/);
}
