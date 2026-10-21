import { expect } from '@playwright/test';
import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.openLaunched(page);
	await Conversation.openTab(page, 'Recruit', 'Email');
});

test('has title', async ({ page }) => {
	await expect(
		page.getByRole('main').getByRole('heading', { name: 'Recruit', level: 1 })
	).toBeVisible();
	await expect(page.getByRole('textbox', { name: 'Emails' })).toBeVisible();
});
