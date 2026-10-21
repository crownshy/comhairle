import { expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { eventually, test } from './utils/testing';
import { exists } from './utils';
import { Textbox } from './utils/components';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Team');
});

test('Configure/team page', async ({ page, cleanup }) => {
	const EmailTextbox = await Textbox('Grant a user "content editor', { page, cleanup });
	const EmailTextboxLabel = page.getByText('Grant a user "content editor');
	const Submit = page.getByRole('button', { name: 'Submit' });

	// Error input
	await EmailTextbox.write('test@crown-shy');
	await Submit.click();
	await eventually(() => expect(EmailTextboxLabel).toHaveAttribute('data-fs-error'));

	// Error user
	await EmailTextbox.write('test@crown-shy.com');
	await Submit.click();
	await eventually(async () => {
		await expect(EmailTextboxLabel).not.toHaveAttribute('data-fs-error');
		await expect(page.getByText('Something went wrong granting').first()).toBeVisible();
	});

	// Valid user
	await EmailTextbox.write('admin@crown-shy.com');
	await Submit.click();
	await eventually(async () =>
		expect(await exists(page.getByRole('cell', { name: 'admin@crown-shy.com' }))).toBe(true)
	);

	const RevokePermission = page.getByRole('button', { name: 'Revoke permission' });
	await RevokePermission.click();

	const Cancel = page.getByRole('button', { name: 'Cancel' });
	await Cancel.click();

	expect(await exists(page.getByRole('cell', { name: 'admin@crown-shy.com' }))).toBe(true);

	await RevokePermission.click();

	const Revoke = page.getByRole('button', { name: 'Revoke', exact: true });
	await Revoke.click();

	await eventually(async () => {
		expect(await exists(page.getByRole('cell', { name: 'admin@crown-shy.com' }))).toBe(false);
	});
});
