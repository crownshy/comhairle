import { test, expect } from '@playwright/test';
import { gotoComhairle, login, openConversation } from './util';

test.beforeEach(async ({ page }) => {
	await gotoComhairle(page);
	await login(page);
	await openConversation(page);
});

test('has title', async ({ page }) => {
	await page.pause();
	expect(page).toHaveTitle('Comhairle');
});

// test('get started link', async ({ page }) => {});
