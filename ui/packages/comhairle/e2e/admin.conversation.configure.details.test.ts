import { expect } from '@playwright/test';
import { eventually, test } from './utils/testing';
import { exists, testWithRefresh } from './utils';
import { login } from './utils/auth';
import { TEST_CONVERSATION_TITLE } from './utils/constants';
import Conversation from './utils/navigation/Conversation';
import { Textbox } from './utils/components';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Details');
});

test('Configure/Details page', async ({ page, cleanup }) => {
	const Title = await Textbox('Title', { page, cleanup }, { resetTo: TEST_CONVERSATION_TITLE });
	const ShortDescription = await Textbox('Short Description', { page, cleanup });
	const Description = await Textbox('Description', { page, cleanup });

	// const primary_language = page.getByRole('button', { name: 'English' });
	// const supported_languages = page.getByRole('textbox', { name: 'Supported languages' });

	// const NEW_PRIMARY_LANGUAGE = 'Arabic';
	// const NEW_SUPPORTED_LANGUAGE = 'Welsh';

	await Title.write();
	await ShortDescription.write();
	await Description.write();

	// await primary_language.click();
	// await page.getByRole('option', { name: NEW_PRIMARY_LANGUAGE }).click();
	// cleanup(async () => {
	// 	await page.getByRole('button', { name: NEW_PRIMARY_LANGUAGE }).first().click();
	// 	await page.getByRole('option', { name: 'English' }).click();
	// 	await page.getByRole('button', { name: 'Remove' }).first().click();
	// });
	//
	// await supported_languages.click();
	// await page.getByRole('button', { name: NEW_SUPPORTED_LANGUAGE }).click();
	// cleanup(async () => {
	// 	await page.getByRole('button', { name: 'Remove' }).first().click();
	// });

	await testWithRefresh(page, async () => {
		await Title.expect();
		await ShortDescription.expect();
		await Description.expect();

		const header = page.getByRole('heading', {
			description: Title.currentValue,
			exact: true
		});
		await eventually(async () => expect(await exists(header)).toBe(true));

		const conversation = page.getByRole('link', { name: Title.currentValue });
		await eventually(async () => expect(await exists(conversation)).toBe(true));

		// Translation stuff
		// await expect(
		// 	page.locator('#conversation-title-field').getByRole('button').filter({ hasText: /^$/ })
		// ).toBeVisible();
		// await expect(page.getByRole('button', { name: 'English approved' })).toBeVisible();
		// await expect(page.getByRole('button', { name: 'Welsh draft' })).toBeVisible();
	});

	// Image stuff
	// await page.getByLabel('Open media library').click();
	// await expect(page.getByLabel('Open media library')).toBeVisible();
	// await page.getByLabel('Open media library').click();
	// await expect(page.getByRole('dialog', { name: 'Media library Go to media' })).toBeVisible();
	// await page.getByRole('button', { name: 'Close', exact: true }).click();
});
