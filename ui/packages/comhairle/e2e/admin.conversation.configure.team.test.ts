import { expect } from '@playwright/test';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { Textbox } from './utils/components';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Team');
});

test('Configure/team page', async ({ page }) => {
	const main = page.getByRole('main');
	// Adding and editing use the same dialog under different titles, and only one is open at a time.
	const dialog = page.getByRole('dialog');
	const Email = await Textbox('Email', { page }, { exact: true });
	const contentEditor = dialog.getByRole('checkbox', { name: /^Content Editor/ });
	const save = dialog.getByRole('button', { name: 'Save changes' });

	const NEW_USER = 'admin@crown-shy.com';
	// Rows are labelled by username, so find the row by its email.
	const editNewUser = main
		.getByRole('listitem')
		.filter({ hasText: NEW_USER })
		.getByRole('button', { name: /^Edit roles for/ });

	// A failed earlier run can leave the role behind.
	await expect(main.getByRole('heading', { name: 'Team', level: 1 })).toBeVisible();
	if (await editNewUser.isVisible()) {
		await removeAllRoles();
	}

	await main.getByRole('button', { name: 'Add people' }).first().click();
	await contentEditor.click();

	await Email.write('not-an-email');
	expect(await Email.isValid()).toBe(false);

	await Email.write('test@crown-shy.com');
	await save.click();
	await expect(dialog.getByRole('alert')).toContainText('Could not save roles');
	await dialog.getByRole('button', { name: 'Cancel' }).click();

	await main.getByRole('button', { name: 'Add people' }).first().click();
	await Email.write(NEW_USER);
	await contentEditor.click();
	await save.click();
	await expect(dialog).toBeHidden();
	await expect(editNewUser).toBeVisible();

	// Cancelling keeps the role.
	await editNewUser.click();
	await expect(contentEditor).toBeChecked();
	await contentEditor.click();
	await dialog.getByRole('button', { name: 'Cancel' }).click();
	await expect(editNewUser).toBeVisible();

	await removeAllRoles();

	async function removeAllRoles() {
		await editNewUser.click();
		const checked = dialog.getByRole('checkbox', { checked: true });
		await expect(checked.first()).toBeVisible();
		while ((await checked.count()) > 0) {
			await checked.first().click();
		}
		await save.click();
		await expect(dialog).toBeHidden();
		await expect(editNewUser).toBeHidden();
	}
});
