import { eventually, test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';
import { Button, NumericTextbox, Textbox, Tiptap } from './utils/components';
import { expect } from '@playwright/test';
import { testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Event Structure');
});

test('Events/Structure page', async ({ page }) => {
	// Add agenda item
	const AddAgendaItem = await Button('+ Add agenda item', page);
	await AddAgendaItem.click();

	const AgendaName = await Textbox('Enter agenda item here', { page });
	await AgendaName.write();

	// Add breakout session
	const AddBreakoutSession = await Button('+ Add breakout session', page);
	await AddBreakoutSession.click();

	const PromptTitle = await Textbox('Enter prompt title here', { page });
	await PromptTitle.write();

	const Time = await NumericTextbox('Time', { page });
	await Time.write(12);

	const GroupSize = await NumericTextbox('Group size', { page });
	await GroupSize.write(8);

	const Instructions = Tiptap(0, { page });
	await Instructions.write();

	const AddBreakoutPrompt = await Button('+ Add breakout prompt', page, { index: 0 });
	await AddBreakoutPrompt.click();

	const Instructions2 = Tiptap(1, { page });
	await Instructions2.write();

	// Group assignment
	// FIX: Currently broken on staging so will come back to it later
	// await page.getByRole('combobox').nth(3).selectOption('diversify');
	// await page.getByRole('checkbox', { name: 'Age' }).check();
	// await page.getByRole('checkbox', { name: 'Gender' }).check();
	// await page.getByRole('checkbox', { name: 'Understanding of AI' }).check();
	// await page.getByRole('checkbox', { name: 'Education' }).check();
	// await page.getByRole('checkbox', { name: 'Postcode' }).check();
	// await page.getByRole('checkbox', { name: 'In-call activities' }).check();

	const RemovePrompt = await Button('Remove prompt', page, { index: 1 });
	await eventually(async () => expect(await RemovePrompt.isVisible()).toBe(true));

	await eventually(async () =>
		expect(await page.getByText('Breakout session').count()).toEqual(2)
	);

	// Remove breakout session
	await RemovePrompt.click();

	const SaveAgenda = await Button('Save Agenda', page);
	await SaveAgenda.click();

	await testWithRefresh(page, async () => {
		await AgendaName.expect();
		await PromptTitle.expect();
		await Time.expect();
		await GroupSize.expect();
		await Instructions.expect();
	});

	// FIX: Need to fix the dragging here
	// await page.pause();
	// const ReorderItem = await Button('Reorder item', page, { index: 1 });
	// await ReorderItem.hover();
	// await page.mouse.down();
	// await (await find(page.getByText('Optimise topics and activities'))).hover();
	// await page.mouse.up();

	// const FirstItem = await find(page.getByRole('group').first());
	// await eventually(() =>
	// 	expect(FirstItem.getByText('Breakout session', { exact: true })).toBeVisible()
	// );

	// const Cards = await find(page.getByRole('group'));
	// await eventually(async () => expect(Cards.count()).toEqual(2));

	const DeleteItem = await Button('Delete item', page);
	await DeleteItem.click();
	await DeleteItem.click();

	await eventually(async () => {
		expect(await page.getByText('Standard item').count()).toEqual(0);
		expect(await page.getByText('Breakout session Time Session').count()).toEqual(0);
	});
});
