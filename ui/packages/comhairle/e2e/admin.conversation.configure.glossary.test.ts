import { test, expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Glossary');
});

test('has title', async ({ page }) => {
	await page.getByRole('textbox', { name: 'Term and synonyms in English' }).click();
	await page.getByRole('textbox', { name: 'Term and synonyms in English' }).fill('test');
	await page.getByRole('textbox', { name: 'Term and synonyms in English' }).press('Tab');
	await page.getByRole('textbox', { name: 'Explanation in English' }).fill('test');
	await expect(page.getByRole('main')).toContainText('Saved');

	await page.getByRole('button', { name: 'Add term' }).click();
	await page.locator('#glossary-terms-1').click();
	await expect(page.getByRole('main')).toContainText('1 term');
	await page.getByRole('button', { name: 'Remove term' }).nth(1).click();
	await page.getByRole('textbox', { name: 'Search terms' }).click();
	await page.getByRole('textbox', { name: 'Search terms' }).fill('test1');
	await expect(page.getByText('No terms match "test1"')).toBeVisible();
	await page.getByRole('textbox', { name: 'Search terms' }).click();
	await page.getByRole('textbox', { name: 'Search terms' }).fill('test');
	await expect(page.getByRole('textbox', { name: 'Term and synonyms in English' })).toBeVisible();
});
