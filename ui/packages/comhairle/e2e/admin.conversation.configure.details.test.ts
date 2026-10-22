import { expect } from '@playwright/test';
import { test } from './utils/testing';
import { testWithRefresh } from './utils';
import { login } from './utils/auth';
import { TEST_CONVERSATION_TITLE } from './utils/constants';
import Conversation from './utils/navigation/Conversation';
import { Switch, Textbox } from './utils/components';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Details');
});

test('Configure/Details page', async ({ page, cleanup }) => {
	const Title = await Textbox(
		'Title',
		{ page, cleanup },
		{ resetTo: TEST_CONVERSATION_TITLE, exact: true }
	);
	const ShortDescription = await Textbox('Short description', { page, cleanup }, { exact: true });
	const Description = await Textbox('Description', { page, cleanup }, { exact: true });

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

		// Translation stuff
		// await expect(
		// 	page.locator('#conversation-title-field').getByRole('button').filter({ hasText: /^$/ })
		// ).toBeVisible();
		// await expect(page.getByRole('button', { name: 'English approved' })).toBeVisible();
		// await expect(page.getByRole('button', { name: 'Welsh draft' })).toBeVisible();
	});

	// Autosave doesn't reload the page, so the sidebar heading only shows the new title after
	// the reload above.
	await expect(
		page.getByRole('heading', { name: Title.currentValue(), exact: true, level: 1 })
	).toBeVisible();

	// Image stuff
	// await page.getByLabel('Open media library').click();
	// await expect(page.getByLabel('Open media library')).toBeVisible();
	// await page.getByLabel('Open media library').click();
	// await expect(page.getByRole('dialog', { name: 'Media library Go to media' })).toBeVisible();
	// await page.getByRole('button', { name: 'Close', exact: true }).click();
});

test('Configure/Details access settings', async ({ page, cleanup }) => {
	const ShowConversationPublicy = await Switch('Show conversation publicly', { page, cleanup });
	const OnlyAllowParticipationBy = await Switch('Only allow participation by', { page, cleanup });
	const AutomaticallyLogInWithAn = await Switch('Automatically log in with an', {
		page,
		cleanup
	});
	const EnableSignupPrompts = await Switch('Enable signup prompts', { page, cleanup });
	const ShowThankYouPageAnonymous = await Switch('Show thank you page anonymous', {
		page,
		cleanup
	});
	const ShowThankYouPageFeedback = await Switch('Show thank you page feedback', {
		page,
		cleanup
	});
	const AllowRevisitAfterFinishing = await Switch('Allow revisit after finishing', {
		page,
		cleanup
	});

	await ShowConversationPublicy.toggle();
	await OnlyAllowParticipationBy.toggle();
	await AutomaticallyLogInWithAn.toggle();
	await EnableSignupPrompts.toggle();
	await ShowThankYouPageAnonymous.toggle();
	await ShowThankYouPageFeedback.toggle();
	await AllowRevisitAfterFinishing.toggle();

	await testWithRefresh(page, async () => {
		await ShowConversationPublicy.expect();
		await OnlyAllowParticipationBy.expect();
		await AutomaticallyLogInWithAn.expect();
		await EnableSignupPrompts.expect();
		await ShowThankYouPageAnonymous.expect();
		await ShowThankYouPageFeedback.expect();
		await AllowRevisitAfterFinishing.expect();
	});
});
