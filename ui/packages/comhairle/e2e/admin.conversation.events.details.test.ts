import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events, { TEST_EVENT_NAME } from './utils/navigation/Events';
import { RadioButtons, Textbox, TimeRangePicker } from './utils/components';
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
	const Capacity = await Textbox(
		'Capacity',
		{ page, cleanup },
		{ overrideRole: 'spinbutton', resetTo: '2' }
	);
	// const DefaultTimezone = Combobox('Europe/London', { page, cleanup });
	// const Date = Calendar('Event date', { page, cleanup });
	const Time = TimeRangePicker({ start: '03:00 AM', end: '04:00 AM' }, { page, cleanup });
	const SignupMode = await RadioButtons(['Invite', 'Open'], { page, cleanup });
	const SaveChanges = page.getByRole('button', { name: 'Save changes' });

	cleanup(async () => {
		await SaveChanges.click();
	});

	await Name.write();
	await Description.write();
	await Capacity.write('10');
	await Time.fill('03:35 AM', '05:00 AM');
	await SignupMode.select('Open');

	await SaveChanges.click();

	await testWithRefresh(page, async () => {
		await Name.expect();
		await Description.expect();
		await Capacity.expect();
		await SignupMode.expect();
	});

	await expect(page.getByRole('heading', { name: Name.value() })).toBeVisible();
});
