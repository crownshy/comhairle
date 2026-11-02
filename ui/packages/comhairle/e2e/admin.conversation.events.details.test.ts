import { eventually, test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events, { TEST_EVENT_NAME } from './utils/navigation/Events';
import { NumericTextbox, RadioButtons, Select, Textbox } from './utils/components';
import { testWithRefresh } from './utils';
import { expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Details');
});

test('Events/Details page', async ({ page, cleanup }) => {
	const Name = await Textbox('Name', { page, cleanup }, { resetTo: TEST_EVENT_NAME });
	const Description = await Textbox('Description', { page });
	const Capacity = await NumericTextbox('Capacity', { page, cleanup }, { resetTo: 2 });
	// const DefaultTimezone = Combobox('Europe/London', { page, cleanup });
	// const Date = Calendar('Event date', { page, cleanup });
	// const Time = TimeRangePicker({ start: '03:00 AM', end: '04:00 AM' }, { page, cleanup });
	const Format = Select(['Online', 'In-person'], { page, cleanup });
	const SignupMode = await RadioButtons(
		['Invite', 'Open'],
		{ page, cleanup },
		{ resetTo: 'Invite' }
	);
	const SaveChanges = page.getByRole('button', { name: 'Save changes' });

	cleanup(async () => {
		await SaveChanges.click();
	});

	await Name.write();
	await Description.write();
	await Capacity.write(10);
	// await Time.fill('03:35 AM', '05:00 AM');
	await Format.pick('In-person');
	await SignupMode.select('Open');

	await SaveChanges.click();

	await testWithRefresh(page, async () => {
		await Name.expect();
		await Description.expect();
		await Capacity.expect();
		await Format.expect();
		// Should be disabled as Format is "In-person"
		await eventually(() =>
			expect(page.getByRole('textbox', { name: 'Custom event link Override' })).toBeDisabled()
		);
		await SignupMode.expect();
	});

	await eventually(() => expect(page.getByRole('heading', { name: Name.value() })).toBeVisible());
	await eventually(() => expect(page.getByRole('link', { name: Name.value() })).toBeVisible());
});
