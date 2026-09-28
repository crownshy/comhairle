import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { generateValue, testWithRefresh } from './utils';
import { test } from './utils/testing';
import { expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Glossary');
});

test('Configure/glossary page', async ({ page }) => {
	const value1 = generateValue();
	const value2 = generateValue();

	const glossaryEditorTerm = page.getByRole('textbox', { name: 'Term and synonyms in English' });
	await glossaryEditorTerm.click();
	await glossaryEditorTerm.fill(value1);
	await glossaryEditorTerm.press('Tab');

	const glossaryEditorExplanation = page.getByRole('textbox', { name: 'Explanation in English' });
	await glossaryEditorExplanation.fill(value2);

	await testWithRefresh(page, async () => {
		await expect(glossaryEditorTerm).toHaveValue(value1);
		await expect(glossaryEditorExplanation).toHaveValue(value2);
	});

	await page.getByRole('button', { name: 'Add term' }).click();
	await page.locator('#glossary-terms-1').click();
	await expect(page.getByRole('main')).toContainText('1 term');
	await page.getByRole('button', { name: 'Remove term' }).nth(1).click();
	await page.getByRole('textbox', { name: 'Search terms' }).click();

	const incorrectSearchTerm = value1 + 'a';
	await page.getByRole('textbox', { name: 'Search terms' }).fill(incorrectSearchTerm);
	await expect(page.getByText(`No terms match "${incorrectSearchTerm}"`)).toBeVisible();
	await page.getByRole('textbox', { name: 'Search terms' }).click();
	await page.getByRole('textbox', { name: 'Search terms' }).fill(value1);
	await expect(glossaryEditorTerm).toBeVisible();
});
