import { test, expect } from './fixtures/auth';

// Happy-path lifecycle: create a conversation via the admin UI, land on configure,
// then walk the section sub-routes (design / monitor / report / events / invites).
//
// NOTE: this test creates real data — point CI at a disposable env, not production.
// No cleanup yet; add a teardown once an API delete-conversation hook is wired up.

const stamp = () => `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;

test.describe('conversation lifecycle', () => {
	test('admin can create a conversation and reach configure', async ({ adminPage: page }) => {
		await page.goto('/admin/conversations/new');

		const title = `e2e smoke conversation ${stamp()}`;
		const desc = `e2e short description for smoke test ${stamp()}`;

		await page.locator('input[name="title"]').fill(title);
		await page.locator('input[name="short_description"]').fill(desc);

		// 'empty' template is selected by default — no extra click needed.
		await Promise.all([
			page.waitForURL(/\/admin\/conversations\/[^/]+\/configure/, { timeout: 30_000 }),
			page.locator('form button[type="submit"]').click()
		]);

		// Capture id for downstream section sweeps within this test.
		const match = page.url().match(/\/admin\/conversations\/([^/]+)\/configure/);
		expect(match, 'expected configure URL with conversation id').not.toBeNull();
		const conversationId = match![1];

		const sections = [
			'configure',
			'design',
			'monitor',
			'report',
			'events',
			'invites',
			'moderate'
		];
		for (const section of sections) {
			const res = await page.goto(`/admin/conversations/${conversationId}/${section}`);
			expect(res!.status(), `${section} returned 5xx`).toBeLessThan(500);
		}
	});

	test('new conversation form rejects too-short title', async ({ adminPage: page }) => {
		await page.goto('/admin/conversations/new');
		await page.locator('input[name="title"]').fill('short');
		await page.locator('input[name="short_description"]').fill('also too short');
		await page.locator('form button[type="submit"]').click();

		// Stay on /new — submit blocked by client-side validation.
		await expect(page).toHaveURL(/\/admin\/conversations\/new/);
	});
});
