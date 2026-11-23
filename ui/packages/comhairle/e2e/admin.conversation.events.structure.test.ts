import { eventually, test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';
import { Button, NumericTextbox, Textbox, Tiptap } from './utils/components';
import { expect } from '@playwright/test';
import { find, testWithRefresh } from './utils';
import type { Page } from './utils/types';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Event Structure');
});

/** A breakout's room settings are collapsed after a reload. */
async function openRoomSettings(page: Page, position: number) {
	const toggle = await Button(`Room settings for item ${position}`, page);
	if ((await toggle.getAttribute('aria-expanded')) !== 'true') await toggle.click();
}

async function waitForSave(page: Page) {
	await eventually(async () =>
		expect(page.getByRole('main').getByText('Saved', { exact: true })).toBeVisible()
	);
}

async function deleteRow(page: Page, title: string) {
	await (await Button(`Options for ${title}`, page)).click();
	await (await find(page.getByRole('menuitem', { name: 'Delete' }))).click();
}

/** Removes the test's rows if a failed run left them behind. */
async function deleteRowIfPresent(page: Page, title: string) {
	const options = page.getByRole('main').getByRole('button', { name: `Options for ${title}` });
	if ((await options.count()) === 0) return;
	await deleteRow(page, title);
	await waitForSave(page);
}

test('Events/Structure page', async ({ page, cleanup }) => {
	await (await Button('Agenda item', page)).click();
	const AgendaTitle = await Textbox('Title of item 1', { page });
	await AgendaTitle.write();
	const AgendaMinutes = await NumericTextbox('Minutes for item 1', { page });
	await AgendaMinutes.write(5);

	await (await Button('Breakout session', page)).click();
	const Question = await Textbox('Question for item 2', { page });
	await Question.write();
	const BreakoutMinutes = await NumericTextbox('Minutes for item 2', { page });
	await BreakoutMinutes.write(12);
	await openRoomSettings(page, 2);
	const GroupSize = await NumericTextbox('People per room', { page });
	await GroupSize.write(8);
	const Instructions = Tiptap(0, { page });
	await Instructions.write();

	cleanup(async () => {
		await deleteRowIfPresent(page, Question.value());
		await deleteRowIfPresent(page, AgendaTitle.value());
	});

	await waitForSave(page);
	await eventually(async () =>
		expect(page.getByRole('main').getByText('17 min · ends')).toBeVisible()
	);

	await testWithRefresh(page, async () => {
		await AgendaTitle.expect();
		await AgendaMinutes.expect();
		await Question.expect();
		await BreakoutMinutes.expect();

		await openRoomSettings(page, 2);
		await GroupSize.expect();
		await Instructions.expect();
	});

	await deleteRow(page, AgendaTitle.value());
	await deleteRow(page, Question.value());
	await waitForSave(page);

	await page.reload();
	await eventually(async () =>
		expect(page.getByRole('main').getByText('No agenda yet')).toBeVisible()
	);
});
