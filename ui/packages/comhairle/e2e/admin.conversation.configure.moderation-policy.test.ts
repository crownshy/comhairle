import { test } from './utils/testing';
import Conversation from './utils/navigation/Conversation';
import { login } from './utils/auth';
import { expect } from '@playwright/test';
import { testWithRefresh } from './utils';
import { Textbox } from './utils/components';

const DEFAULT_REASON_COUNT = 5;

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Moderation policy');
});

test('Configure/Moderation-Policy page', async ({ page }) => {
	const main = page.getByRole('main');
	const usingDefaults = main.getByText('This conversation uses the default reasons');
	const resetToDefault = main.getByRole('button', { name: 'Reset to default' });
	const reasons = main.getByRole('textbox', { name: 'Reason', exact: true });

	// A failed earlier run can leave this conversation on its own list.
	if (await resetToDefault.isVisible()) {
		await resetToDefault.click();
	}
	await expect(usingDefaults).toBeVisible();
	await expect(reasons).toHaveCount(DEFAULT_REASON_COUNT);

	const addReason = main.getByRole('button', { name: 'Add reason' });
	await addReason.click();
	await expect(reasons).toHaveCount(DEFAULT_REASON_COUNT + 1);

	const NewReason = await Textbox(
		'Reason',
		{ page },
		{ index: DEFAULT_REASON_COUNT, exact: true }
	);
	const NewDescription = await Textbox(
		'What counts under this reason',
		{ page },
		{ index: DEFAULT_REASON_COUNT }
	);
	await NewReason.write();
	await NewDescription.write();
	await expect(main.getByText('Saved')).toBeVisible();

	await testWithRefresh(page, async () => {
		await NewReason.expect();
		await NewDescription.expect();
	});

	await main.getByRole('button', { name: 'Remove reason' }).last().click();

	await testWithRefresh(page, async () => {
		await expect(reasons).toHaveCount(DEFAULT_REASON_COUNT);
		await expect(reasons.last()).not.toHaveValue(NewReason.currentValue());
	});

	await addReason.click();
	await NewReason.write();
	await NewDescription.write();
	await expect(main.getByText('Saved')).toBeVisible();

	await resetToDefault.click();
	await expect(usingDefaults).toBeVisible();
	await expect(reasons).toHaveCount(DEFAULT_REASON_COUNT);
});
