import { expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { exists } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Team');
});

test('Configure/team page', async ({ page }) => {
	const EmailTextbox = page.getByRole('textbox', { name: 'Grant a user "content editor' });
	const EmailTextboxLabel = page.getByText('Grant a user "content editor');
	const Submit = page.getByRole('button', { name: 'Submit' });

	const ERROR_INPUT = 'test@crown-shy';
	await EmailTextbox.click();
	await EmailTextbox.fill(ERROR_INPUT);
	await Submit.click();

	await expect(EmailTextboxLabel).toHaveAttribute('data-fs-error');

	const ERROR_USER = 'test@crown-shy.com';
	await EmailTextbox.click();
	await EmailTextbox.fill(ERROR_USER);
	await Submit.click();

	await expect(EmailTextboxLabel).not.toHaveAttribute('data-fs-error');
	await expect(page.getByText('Something went wrong granting').first()).toBeVisible();

	const NEW_USER = 'admin@crown-shy.com';
	await EmailTextbox.click();
	await EmailTextbox.fill(NEW_USER);
	await Submit.click();

	expect(await exists(page.getByRole('cell', { name: 'admin@crown-shy.com' }))).toBe(true);

	const RevokePermission = page.getByRole('button', { name: 'Revoke permission' });
	await RevokePermission.click();

	const Cancel = page.getByRole('button', { name: 'Cancel' });
	await Cancel.click();

	expect(await exists(page.getByRole('cell', { name: 'admin@crown-shy.com' }))).toBe(true);

	await RevokePermission.click();

	const Revoke = page.getByRole('button', { name: 'Revoke', exact: true });
	await Revoke.click();

	expect(await exists(page.getByRole('cell', { name: 'admin@crown-shy.com' }))).toBe(false);
});
