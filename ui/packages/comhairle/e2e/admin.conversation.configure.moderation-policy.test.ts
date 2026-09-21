import { eventually, test } from './utils/testing';
import Conversation from './utils/navigation/Conversation';
import { login } from './utils/auth';
import { expect } from '@playwright/test';
import { exists, sleep, testWithRefresh } from './utils';
import Textboxes from './utils/inputs/Textboxes';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Moderation policy');
});

test('Configure/Moderation-Policy page', async ({ page }) => {
	const Info = page.getByText('This conversation uses the');

	expect(await exists(Info)).toBe(true);

	const AddReason = page.getByRole('button', { name: 'Add reason' });
	await AddReason.click();

	const NewReason = page.getByRole('textbox', { name: 'Reason' }).nth(10);
	let newReason = await Textboxes.write(NewReason);
	await NewReason.press('Tab');

	const NewDescription = page
		.getByRole('textbox', { name: 'What counts under this reason' })
		.nth(5);
	let newDescription = await Textboxes.write(NewDescription);

	await eventually(async () => {
		await expect(page.getByText('Saved')).toBeVisible();
	});

	await testWithRefresh(page, async () => {
		await expect(NewReason).toHaveValue(newReason);
		await expect(NewDescription).toHaveValue(newDescription);
	});

	const RemoveNewReason = page.getByRole('button', { name: 'Remove reason' }).nth(5);
	await RemoveNewReason.click();

	await testWithRefresh(page, async () => {
		expect(await exists(NewReason)).toBe(false);
		expect(await exists(NewDescription)).toBe(false);
	});

	await AddReason.click();

	newReason = await Textboxes.write(NewReason);
	newDescription = await Textboxes.write(NewDescription);

	await sleep(1.5);

	const ResetToDefault = page.getByRole('button', { name: 'Reset to default' });
	await ResetToDefault.click();

	expect(await exists(Info)).toBe(true);
});
