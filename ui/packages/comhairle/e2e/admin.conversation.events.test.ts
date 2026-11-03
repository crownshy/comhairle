import { expect } from '@playwright/test';
import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
});

test('has title', async ({ page }) => {
	await expect(
		page.getByRole('main').getByRole('heading', { name: 'Events', level: 1 })
	).toBeVisible();
});

test('Events page - Add event', async ({ page }) => {
	await Events.add({ page, withTests: true });
});
