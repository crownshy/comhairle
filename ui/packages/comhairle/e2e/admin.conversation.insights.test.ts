import { expect } from '@playwright/test';
import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.openLaunched(page);
	await Conversation.openTab(page, 'Insights', undefined);
});

test('has title', async ({ page }) => {
	await expect(
		page.getByRole('main').getByRole('heading', { name: 'Insights', level: 1 })
	).toBeVisible();
});
