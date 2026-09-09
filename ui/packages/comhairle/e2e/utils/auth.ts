import { m } from '../../src/lib/paraglide/messages';
import type { Page } from './types';
import { exists } from '.';

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
	await email.fill(process.env.PLAYWRIGHT_ADMIN_USERNAME ?? '');
	await email.press('Tab');

	await page
		.getByRole('textbox', { name: 'Password' })
		.fill(process.env.PLAYWRIGHT_ADMIN_PASSWORD ?? '');
	await page.getByRole('button', { name: 'Log In' }).click();

	await page.waitForURL('**/admin');
}
