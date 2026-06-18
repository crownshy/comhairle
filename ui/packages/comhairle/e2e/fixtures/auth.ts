import { test as base, expect, type Page } from '@playwright/test';

// Credentials come from env so we never commit secrets. Set in .env.test or CI secrets:
//   TEST_ADMIN_EMAIL=...
//   TEST_ADMIN_PASSWORD=...
//   TEST_USER_EMAIL=...        (optional, non-admin)
//   TEST_USER_PASSWORD=...
//
// Tests that require a logged-in fixture auto-skip when the matching env is missing,
// so the suite stays green in setups that haven't provisioned test accounts yet.

export async function loginVia(page: Page, email: string, password: string) {
	await page.goto('/auth/login');
	await page.locator('input[name="email"]').fill(email);
	await page.locator('input[name="password"]').fill(password);
	await Promise.all([
		page.waitForURL((url) => !/\/auth\/login/.test(url.pathname), { timeout: 15_000 }),
		page.locator('form button[type="submit"]').click()
	]);
	// Sanity: an error message under the form means creds were rejected.
	await expect(page.locator('.text-destructive')).toHaveCount(0);
}

type AuthFixtures = {
	adminPage: Page;
	userPage: Page;
};

export const test = base.extend<AuthFixtures>({
	adminPage: async ({ browser }, use) => {
		const email = process.env.TEST_ADMIN_EMAIL;
		const password = process.env.TEST_ADMIN_PASSWORD;
		test.skip(!email || !password, 'TEST_ADMIN_EMAIL/PASSWORD not set');
		const ctx = await browser.newContext();
		const page = await ctx.newPage();
		await loginVia(page, email!, password!);
		await use(page);
		await ctx.close();
	},

	userPage: async ({ browser }, use) => {
		const email = process.env.TEST_USER_EMAIL;
		const password = process.env.TEST_USER_PASSWORD;
		test.skip(!email || !password, 'TEST_USER_EMAIL/PASSWORD not set');
		const ctx = await browser.newContext();
		const page = await ctx.newPage();
		await loginVia(page, email!, password!);
		await use(page);
		await ctx.close();
	}
});

export { expect };
