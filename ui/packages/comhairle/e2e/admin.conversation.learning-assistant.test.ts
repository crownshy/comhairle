import { expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { Textbox, Switch } from './utils/components';
import { eventually, test } from './utils/testing';
import { exists, sleep, testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Learning Assistant', undefined);
	await sleep(1);
});

test('Learning Assistant page', async ({ page, cleanup }) => {
	// Show Learning Assistant
	const ShowLearningAssistant = await Switch('Show Learning Assistant', { page, cleanup });
	await ShowLearningAssistant.toggle();

	await testWithRefresh(page, async () => {
		await ShowLearningAssistant.expect();
	});

	// Learn Content
	await page.getByRole('button', { name: 'Sync learn content' }).click();
	await eventually(() => expect(page.getByText('Processing synced learn')).toBeVisible());
	await eventually(() => expect(page.getByText('Learn content synced and ready')).toBeVisible());

	// Documents
	// Can't do it, because can't upload documents?

	// Cross-language Search
	const SupportedLanguages = page.getByRole('textbox', { name: 'Supported languages' });
	await SupportedLanguages.click();
	await page.getByRole('button', { name: 'English' }).click();
	await sleep(0.5);
	await SupportedLanguages.click();
	await page.getByRole('button', { name: 'Gaelic' }).click();
	await testWithRefresh(page, () =>
		eventually(async () => {
			expect(await exists(page.getByText('English'))).toBe(true);
			expect(await exists(page.getByText('Gaelic'))).toBe(true);
		})
	);

	await page.getByRole('button', { name: 'Remove' }).first().click();
	await page.getByRole('button', { name: 'Remove' }).first().click();
	await testWithRefresh(page, () =>
		eventually(async () => {
			expect(await exists(page.getByText('English'))).toBe(false);
			expect(await exists(page.getByText('Gaelic'))).toBe(false);
		})
	);

	// Target reading age
	const TargetReadingAge = await Textbox(
		undefined,
		{ page, cleanup },
		{ resetTo: '9', overrideRole: 'spinbutton' }
	);

	await TargetReadingAge.write('18');

	await TargetReadingAge.write('19');
	expect(await TargetReadingAge.isValid()).toBe(false);
	await expect(page.getByText('Something went wrong updating').first()).toBeVisible();

	await TargetReadingAge.write('10');
	expect(await TargetReadingAge.isValid()).toBe(true);

	await TargetReadingAge.write('4');
	expect(await TargetReadingAge.isValid()).toBe(false);
	await expect(page.getByText('Something went wrong updating').first()).toBeVisible();

	await TargetReadingAge.write('8');

	await testWithRefresh(page, async () => {
		await TargetReadingAge.expect();
	});
});
