import { expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Textboxes from './utils/inputs/Textboxes';
import Switches from './utils/inputs/Switches';
import { test } from './utils/testing';
import { exists, sleep, testWithRefresh } from './utils';
import { Minute, Second } from '../src/lib/utils/units';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Learning Assistant', undefined);
});

test('Learning Assistant page', async ({ page, cleanup }) => {
	// Show Learning Assistant
	const switches = await Switches([['show_learning_assistant', 'Show Learning Assistant']], {
		page,
		cleanup
	});

	await switches.toggle('show_learning_assistant');

	await testWithRefresh(page, async () => {
		await switches.expect();
	});

	// Learn Content
	await page.getByRole('button', { name: 'Sync learn content' }).click();
	await sleep(1.5);
	await expect(page.getByText('Processing synced learn')).toBeVisible();
	await expect(async () => {
		await expect(page.getByText('Learn content synced and ready')).toBeVisible();
	}).toPass({
		intervals: [5 * Second],
		timeout: 1 * Minute
	});

	// Documents

	// Cross-language Search
	await page.getByRole('textbox', { name: 'Supported languages' }).click();
	await page.getByRole('button', { name: 'English' }).click();
	await page.getByRole('button', { name: 'Gaelic' }).click();
	await testWithRefresh(page, async () => {
		expect(await exists(page.getByText('English'))).toBe(true);
		expect(await exists(page.getByText('Gaelic'))).toBe(true);
	});

	await page.getByRole('button', { name: 'Remove' }).first().click();
	await page.getByRole('button', { name: 'Remove' }).first().click();
	await testWithRefresh(page, async () => {
		expect(await exists(page.getByText('English'))).toBe(false);
		expect(await exists(page.getByText('Gaelic'))).toBe(false);
	});

	// Target reading age
	const TargetReadingAge = page.getByRole('spinbutton');

	await TargetReadingAge.click();
	await TargetReadingAge.fill('18');

	cleanup(async () => {
		await TargetReadingAge.fill('9');
	});

	await TargetReadingAge.fill('19');
	expect(await Textboxes.isValid(TargetReadingAge)).toBe(false);
	await expect(page.getByText('Something went wrong updating').first()).toBeVisible();

	await TargetReadingAge.fill('10');
	expect(await Textboxes.isValid(TargetReadingAge)).toBe(true);

	await TargetReadingAge.fill('4');
	expect(await Textboxes.isValid(TargetReadingAge)).toBe(false);
	await expect(page.getByText('Something went wrong updating').first()).toBeVisible();

	await TargetReadingAge.fill('9');

	await testWithRefresh(page, async () => {
		expect(TargetReadingAge).toHaveValue('9');
	});
});
