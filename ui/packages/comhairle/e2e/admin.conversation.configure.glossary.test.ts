import { test, expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversations from './utils/navigation/Conversations';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversations.open(page);
	await Conversations.openTab(page, 'Configure', 'Glossary');
});

test('has title', async ({ page }) => {
	await page.pause();
	expect(true).toBe(true);
});
