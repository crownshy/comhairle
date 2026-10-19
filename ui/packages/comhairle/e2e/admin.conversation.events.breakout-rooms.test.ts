import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';
import { Button } from './utils/components';
import { testWithRefresh } from './utils';
import { expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Breakout Rooms');
});

// TODO: Flesh this out more, need to add tests with multiple partipicants, reshuffle, etc
test('Events/Breakout-rooms page', async ({ page }) => {
	const AutoAssign = await Button('Auto-assign', page);

	await AutoAssign.click();

	await testWithRefresh(page, () =>
		expect(page.getByText('Room #1 admin@crown-shy.com')).toBeVisible()
	);
});
