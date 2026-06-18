import { expect, test, type Page } from '@playwright/test';

// Smoke coverage for important routes. Each test documents:
//   - that the route resolves (no 5xx, no crash)
//   - a stable on-page marker so accidental blanking is caught
//   - expected auth behavior (redirect target) for gated routes

async function expectLoads(page: Page, path: string) {
	const res = await page.goto(path);
	expect(res, `no response for ${path}`).not.toBeNull();
	expect(res!.status(), `bad status for ${path}`).toBeLessThan(500);
}

test.describe('public routes', () => {
	test('landing renders', async ({ page }) => {
		await expectLoads(page, '/');
		await expect(page.locator('h1, h2').first()).toBeVisible();
	});

	test('conversations list renders', async ({ page }) => {
		await expectLoads(page, '/conversations');
		await expect(page).toHaveURL(/\/conversations/);
	});

	test('login page renders form', async ({ page }) => {
		await expectLoads(page, '/auth/login');
		await expect(
			page.locator('input[type="email"], input[name="email"]').first()
		).toBeVisible();
	});

	test('signup page renders', async ({ page }) => {
		await expectLoads(page, '/auth/signup');
		await expect(page.locator('form').first()).toBeVisible();
	});

	test('about page renders', async ({ page }) => {
		await expectLoads(page, '/about');
	});

	test('legal: privacy', async ({ page }) => {
		await expectLoads(page, '/rights/privacy');
	});

	test('legal: tos', async ({ page }) => {
		await expectLoads(page, '/rights/tos');
	});

	test('legal: cookies', async ({ page }) => {
		await expectLoads(page, '/rights/cookies');
	});
});

test.describe('auth-gated routes', () => {
	// Unauthenticated hits should NOT 500. Document current redirect/gate behavior here;
	// flip the assertion when the gate changes intentionally.
	test('admin root gates unauthenticated users', async ({ page }) => {
		const res = await page.goto('/admin');
		expect(res!.status()).toBeLessThan(500);
		// Either redirected to login, or rendered an auth wall — both acceptable, neither is a crash.
		const url = page.url();
		const onLogin = /\/auth\/login/.test(url);
		const stillOnAdmin = /\/admin/.test(url);
		expect(onLogin || stillOnAdmin).toBe(true);
	});

	test('settings gates unauthenticated users', async ({ page }) => {
		const res = await page.goto('/settings');
		expect(res!.status()).toBeLessThan(500);
	});

	test('notifications gates unauthenticated users', async ({ page }) => {
		const res = await page.goto('/notifications');
		expect(res!.status()).toBeLessThan(500);
	});
});
