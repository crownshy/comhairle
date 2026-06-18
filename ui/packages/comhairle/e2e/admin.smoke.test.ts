import { test, expect } from './fixtures/auth';

// Smoke coverage for admin-only routes. Requires TEST_ADMIN_EMAIL/PASSWORD env.
// Each test documents the route, asserts it loads without crashing, and pins one stable marker.

test.describe('admin routes (authenticated)', () => {
	test('admin landing renders', async ({ adminPage: page }) => {
		const res = await page.goto('/admin');
		expect(res!.status()).toBeLessThan(500);
		await expect(page).toHaveURL(/\/admin/);
	});

	test('conversations list renders', async ({ adminPage: page }) => {
		await page.goto('/admin/conversations');
		await expect(page).toHaveURL(/\/admin\/conversations/);
	});

	test('new conversation form renders', async ({ adminPage: page }) => {
		await page.goto('/admin/conversations/new');
		await expect(page.locator('form, input').first()).toBeVisible();
	});

	test('info tools landing renders', async ({ adminPage: page }) => {
		const res = await page.goto('/admin/info/tools');
		expect(res!.status()).toBeLessThan(500);
	});

	test('sidebar exposes top-level admin sections', async ({ adminPage: page }) => {
		await page.goto('/admin');
		// Sidebar nav: at least the Conversations entry should be reachable.
		const conversationsLink = page.getByRole('link', { name: /conversations/i }).first();
		await expect(conversationsLink).toBeVisible();
	});
});
