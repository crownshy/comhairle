import { expect } from '@playwright/test';
import { test } from './utils/testing';
import Conversation from './utils/navigation/Conversation';
import { Textbox, CollapsibleRichField } from './utils/components';
import { login } from './utils/auth';
import { testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Content');
});

test('Configure/Content page', async ({ page, cleanup }) => {
	const PrivacyPolicy = await CollapsibleRichField('privacy policy', { page, cleanup });
	const ShortPrivacyPolicy = await CollapsibleRichField('short privacy policy', {
		page,
		cleanup
	});
	const Faqs = await CollapsibleRichField('faqs', { page, cleanup });
	const ThankYou = await CollapsibleRichField('thank you message', { page, cleanup });
	const CTA = await Textbox('Call to action', { page, cleanup });

	await PrivacyPolicy.write();
	await ShortPrivacyPolicy.write();
	await Faqs.write();
	await ThankYou.write();
	await CTA.write();

	await testWithRefresh(page, async () => {
		await PrivacyPolicy.expect();
		await ShortPrivacyPolicy.expect();
		await Faqs.expect();
		await ThankYou.expect();
		await CTA.expect();
	});
});

test('Configure/Content glossary', async ({ page }) => {
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
