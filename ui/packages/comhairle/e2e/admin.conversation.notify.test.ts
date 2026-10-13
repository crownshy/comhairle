import { test, expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.launch(page);
	await Conversation.openTab(page, 'Notify', undefined);
});

test('has title', async ({ page }) => {
	await page.pause();
	expect(true).toBe(true);
});
