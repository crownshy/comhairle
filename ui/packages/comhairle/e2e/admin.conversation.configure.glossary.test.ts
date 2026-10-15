import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { testWithRefresh } from './utils';
import { test } from './utils/testing';
import { expect } from '@playwright/test';
import { Textbox } from './utils/components';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Glossary');
});

test('Configure/glossary page', async ({ page }) => {
	const GlossaryEditorTerm = await Textbox('Term and synonyms in English', { page });
	await GlossaryEditorTerm.write();
	await GlossaryEditorTerm.locator().press('Tab');

	const GlossaryEditorExplanation = await Textbox('Explanation in English', { page });
	await GlossaryEditorExplanation.write();

	await testWithRefresh(page, async () => {
		await GlossaryEditorTerm.expect();
		await GlossaryEditorExplanation.expect();
	});

	await page.getByRole('button', { name: 'Add term' }).click();
	await page.locator('#glossary-terms-1').click();
	await expect(page.getByRole('main')).toContainText('1 term');
	await page.getByRole('button', { name: 'Remove term' }).nth(1).click();
	await page.getByRole('textbox', { name: 'Search terms' }).click();

	const incorrectSearchTerm = GlossaryEditorTerm.currentValue() + 'a';
	await page.getByRole('textbox', { name: 'Search terms' }).fill(incorrectSearchTerm);
	await expect(page.getByText(`No terms match "${incorrectSearchTerm}"`)).toBeVisible();
	await page.getByRole('textbox', { name: 'Search terms' }).click();
	await page
		.getByRole('textbox', { name: 'Search terms' })
		.fill(GlossaryEditorTerm.currentValue());
	await expect(GlossaryEditorTerm.locator()).toBeVisible();
});
